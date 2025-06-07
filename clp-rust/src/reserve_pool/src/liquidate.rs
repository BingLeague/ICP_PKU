use crate::memory::Memory;
use crate::token::*;
use crate::types::Account;
use crate::types::Error;
use crate::UserInfo;
use crate::{AssetInfo, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_stable_structures::StableBTreeMap;
use std::collections::HashMap;

// 清算计算结构体
#[derive(Debug)]
struct LiquidationCalc {
    cusd_amount: u128,
    asset_amount: u128,
    cusd_total: u128,
}

impl LiquidationCalc {
    fn new(cusd_amount: u128, asset_amount: u128, cusd_total: u128) -> Self {
        Self {
            cusd_amount,
            asset_amount,
            cusd_total,
        }
    }

    fn calculate_transfer_amounts(&self) -> (u128, u128) {
        if self.cusd_amount > self.cusd_total {
            let transfer_asset = self.cusd_total * self.asset_amount / self.cusd_amount;
            (self.cusd_total, transfer_asset)
        } else {
            (self.cusd_amount, self.asset_amount)
        }
    }
}

// 用户资产更新结构体
struct UserAssetUpdate {
    addr: Principal,
    info: UserInfo,
    cusd_sub: u128,
    asset_add: u128,
}

async fn transfer_token_fn(token: &Token, amount: u128, to: Principal) -> Result<Nat, Error> {
    if amount == 0 {
        return Ok(Nat::from(0u128));
    }
    token
        .transfer(TransferArgsICRC1 {
            amount: Nat::from(amount),
            created_at_time: None,
            fee: None,
            from_subaccount: None,
            memo: None,
            to: Account {
                owner: to,
                subaccount: None,
            },
        })
        .await
        .map_err(|e| match e {
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })
}

fn update_user_assets(
    users: &mut StableBTreeMap<Principal, UserInfo, Memory>,
    underlying: Principal,
    transfer_cusd: u128,
    transfer_asset: u128,
    cusd_total: u128,
) {
    if cusd_total == 0 {
        return;
    }

    let mut updates = Vec::new();

    // 收集需要更新的用户信息
    for (addr, info) in users.iter() {
        if info.balance == 0 {
            continue;
        }
        let cusd_sub = info.balance.checked_mul(transfer_cusd).unwrap() / cusd_total;
        let asset_add = info.balance.checked_mul(transfer_asset).unwrap() / cusd_total;
        updates.push(UserAssetUpdate {
            addr,
            info: info.clone(),
            cusd_sub,
            asset_add,
        });
    }

    // 批量更新用户信息
    for update in updates {
        let mut info = update.info;
        info.total_sub += update.cusd_sub;
        info.balance -= update.cusd_sub;
        match info.underlies.get_mut(&underlying) {
            Some(asset_info) => {
                asset_info.asset_add += update.asset_add;
                asset_info.cusd_sub += update.cusd_sub;
                asset_info.wait_withdrawal += update.asset_add;
            }
            None => {
                info.underlies.insert(
                    underlying,
                    AssetInfo {
                        asset_add: update.asset_add,
                        cusd_sub: update.cusd_sub,
                        wait_withdrawal: update.asset_add,
                    },
                );
            }
        }
        users.insert(update.addr, info);
    }
}

fn update_liquidate_underlying(
    liquidate_underlying: &mut HashMap<Principal, u128>,
    underlying: Principal,
    asset_amount: u128,
    transfer_asset: u128,
) {
    if asset_amount <= transfer_asset {
        return;
    }

    let remaining_asset = asset_amount - transfer_asset;
    liquidate_underlying
        .entry(underlying)
        .and_modify(|amount| *amount += remaining_asset)
        .or_insert(remaining_asset);
}

#[update]
async fn liquidate(
    cusd_amount: u128,
    asset_amount: u128,
    underlying: Principal,
) -> Result<u128, Error> {
    // 1. 验证调用者和初始状态
    let (cusd, cusd_total) = STATE.with(|state| {
        let state = state.borrow();
        if state.vault.unwrap() != api::caller() {
            return Err(Error::Unauthorized);
        }
        Ok((state.cusd.unwrap(), state.balance))
    })?;
    // 2. 计算转账金额
    let calc = LiquidationCalc::new(cusd_amount, asset_amount, cusd_total);
    let (transfer_cusd, transfer_asset) = calc.calculate_transfer_amounts();

    ic_cdk::println!(
        "liquidate: cusd_amount:{} asset_amount:{} cusd_total:{}, transfer_cusd:{} transfer_asset:{}",
        cusd_amount, asset_amount, cusd_total, transfer_cusd, transfer_asset
    );

    // 3. 执行 CUSD 转账
    let token = Token::new(cusd);
    transfer_token_fn(&token, transfer_cusd, api::caller()).await?;

    // 4. 更新状态
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        // 更新总余额
        if cusd_total < cusd_amount {
            state.balance -= cusd_total;
            state.total_sub += cusd_total;
        } else {
            state.balance -= cusd_amount;
            state.total_sub += cusd_amount;
        }
        // 更新用户资产
        update_user_assets(
            &mut state.users,
            underlying,
            transfer_cusd,
            transfer_asset,
            cusd_total,
        );
        // 更新清算资产
        update_liquidate_underlying(
            &mut state.liquidate_underlying,
            underlying,
            asset_amount,
            transfer_asset,
        );
    });

    Ok(transfer_cusd)
}
