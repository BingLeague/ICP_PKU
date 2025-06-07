use crate::{Result, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_cdk_macros::*;
use shard_lib::approve_permission_change;
use shard_lib::constant::SUB_ACCOUNT_PLATFORM_INCOME;
use shard_lib::permission::check_caller;
use shard_lib::permission::*;
use shard_lib::propose_permission_change;
use shard_lib::token::*;
use shard_lib::types::*;
use shard_lib::utils::create_subaccount;
use std::collections::HashSet;
use std::convert::TryInto;

// 提取手续费的通用结构
struct FeeWithdrawal {
    token: Token,
    amount: u128,
    min_fee: u128,
    from_subaccount: Option<Subaccount>,
}

impl FeeWithdrawal {
    async fn execute(self) -> Result<Nat> {
        if self.amount < self.min_fee * 2 {
            return Err(Error::AmountLessThanFee);
        }
        self.token
            .transfer(TransferArgsICRC1 {
                amount: Nat::from(self.amount - self.min_fee),
                created_at_time: None,
                fee: None,
                from_subaccount: self.from_subaccount,
                memo: None,
                to: Account {
                    owner: api::caller(),
                    subaccount: None,
                },
            })
            .await
            .map_err(|e| match e {
                TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
                TxError::InsufficientFunds { balance } => {
                    ic_cdk::println!("balance: {} amount {}", balance, self.amount);
                    Error::InsufficientBalance
                }
                _ => Error::TransferFailure,
            })
    }
}

#[update(name = "permissionProposeChange")]
fn permission_propose_change(
    target: Principal,
    new_role: Role,
    new_allowed_functions: HashSet<String>,
) -> Result<(), String> {
    STATE.with(|state| {
        let caller = api::caller();
        let mut state = state.borrow_mut();
        propose_permission_change(
            &mut state.permission_state,
            caller,
            target,
            new_role,
            new_allowed_functions,
        )
    })
}

#[update(name = "permissionApproveChange")]
pub fn fn_approve_permission_change(target: Principal) -> Result<(), String> {
    STATE.with(|state| {
        let caller = api::caller();
        let mut state = state.borrow_mut();
        approve_permission_change(&mut state.permission_state, caller, target)
    })
}

#[query(name = "getPermission")]
pub fn fn_get_permission(principal: Principal) -> Option<Permission> {
    STATE.with(|state| {
        let state = state.borrow();
        get_permission(&state.permission_state, principal)
    })
}
#[query(name = "getPendingChanges")]
pub fn fn_get_pending_changes() -> Vec<PermissionChange> {
    STATE.with(|state| {
        let state = state.borrow();
        get_pending_changes(&state.permission_state)
    })
}
#[query(name = "getRecentLogs")]
pub fn fn_get_recent_logs(limit: usize) -> Vec<PermissionLog> {
    STATE.with(|state| {
        let state = state.borrow();
        get_recent_logs(&state.permission_state, limit)
    })
}

#[update(name = "withdrawFee")]
async fn withdraw_fee(amount: u128) -> Result<()> {
    // Example: Maximum allowed fee withdrawal
    const MAX_FEE_WITHDRAWAL: u128 = 10_000_000_000_000;
    check_caller(
        STATE.with(|state| state.borrow().permission_state.clone()),
        "withdrawFee",
    )
    .unwrap();
    if amount > MAX_FEE_WITHDRAWAL {
        return Err(Error::ExcessiveWithdrawal);
    }
    // 一次性获取所需状态
    let cusd = STATE.with(|state| {
        let state = state.borrow();
        if amount > state.fee_balance {
            return Err(Error::InsufficientBalance);
        }
        Ok(state.cusd.unwrap())
    })?;

    let token = Token::new(cusd);
    let fee = token.fee().await.0.try_into().unwrap();
    // 执行提款
    FeeWithdrawal {
        token,
        amount,
        min_fee: fee,
        from_subaccount: create_subaccount(SUB_ACCOUNT_PLATFORM_INCOME),
    }
    .execute()
    .await?;

    // 更新状态
    STATE.with(|state| {
        state.borrow_mut().fee_balance -= amount;
    });

    Ok(())
}

#[update(name = "withdrawUnderlyingFee")]
async fn withdraw_underlying_fee(underlying: Principal, amount: u128) -> Result<()> {
    check_caller(
        STATE.with(|state| state.borrow().permission_state.clone()),
        "withdrawUnderlyingFee",
    )
    .unwrap();

    // 一次性获取所需状态
    let fee = STATE.with(|state| {
        let state = state.borrow();
        match state.underlying.get(&underlying) {
            Some(info) => {
                if amount < info.fee_balance || amount < info.fee * 2 {
                    return Err(Error::InsufficientBalance);
                }
                Ok(info.fee)
            }
            None => Err(Error::InsufficientBalance),
        }
    })?;

    // 执行提款
    let token = Token::new(underlying);
    FeeWithdrawal {
        token,
        amount,
        min_fee: fee,
        from_subaccount: None,
    }
    .execute()
    .await?;

    // 更新状态
    STATE.with(|state| {
        state
            .borrow_mut()
            .underlying
            .get_mut(&underlying)
            .unwrap()
            .fee_balance -= amount;
    });

    Ok(())
}

#[update(name = "withdrawCusdFee")]
async fn withdraw_cusd_fee(amount: u128) -> Result<()> {
    check_caller(
        STATE.with(|state| state.borrow().permission_state.clone()),
        "withdrawCusdFee",
    )
    .unwrap();

    // 一次性获取所需状态
    let (cusd, current_withdrawn_fee) = STATE.with(|state| {
        let state = state.borrow();
        Ok((state.cusd.unwrap(), state.withdrawn_cusd_fee))
    })?;

    let token = Token::new(cusd);
    let total_fee: u128 = token.fee_balance().await.0.try_into().unwrap();

    if amount + current_withdrawn_fee > total_fee {
        return Err(Error::InsufficientBalance);
    }

    // 执行提款
    FeeWithdrawal {
        token,
        amount,
        min_fee: 0, // CUSD fee 不需要最小值检查
        from_subaccount: None,
    }
    .execute()
    .await?;

    // 更新状态
    STATE.with(|state| {
        state.borrow_mut().withdrawn_cusd_fee += amount;
    });

    Ok(())
}

#[update]
async fn burn(amount: u128) -> Result<()> {
    let cusd = STATE.with(|state| {
        let state = state.borrow();
        if amount > state.excessive_cusd {
            Err(Error::InsufficientBalance)
        } else {
            Ok(state.cusd.unwrap())
        }
    })?;
    let token = Token::new(cusd);
    let fee: u128 = token.fee().await.0.try_into().unwrap();
    if amount < fee {
        return Err(Error::AmountLessThanFee);
    }
    token
        .transfer_from(TransferArgs {
            amount: Nat::from(amount),
            created_at_time: None,
            fee: None,
            memo: None,
            spender_subaccount: None,
            from: Account {
                owner: api::caller(),
                subaccount: None,
            },
            to: Account {
                owner: api::id(),
                subaccount: None,
            },
        })
        .await
        .map(|_| {
            STATE.with(|state| {
                state.borrow_mut().excessive_cusd -= amount;
                Ok(())
            })
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?
}
