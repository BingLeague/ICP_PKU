use crate::set::_new_reward_per_token;
use crate::token::{Token, TransferArgsICRC1};
use crate::{Account, AssetConfig, Result, UserAssetIdentify, STATE};
use crate::{Error, SECONDS};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use std::ops::{Add, Div, Mul, Sub};

pub const MINING_MUL: u128 = 100000000000;

#[update(name = "update_balance", guard = "check_caller")]
pub fn update_balance(user: Principal, asset: Account, amount: u128, is_add: bool) -> Result<()> {
    let identify = UserAssetIdentify {
        user,
        underlying: asset.clone(),
    };
    let now = api::time() / SECONDS;
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let clpt_price = state.clpt_price.clone();
        let mut info = state.users.get(&identify).unwrap_or_default();
        let amount = {
            if !is_add && amount > info.balance {
                info.balance
            } else {
                amount
            }
        };
        match state.asset_config.get_mut(&asset) {
            Some(config) => {
                let new_per_token = _new_reward_per_token(config);
                if config.price > 0 && clpt_price > 0 && config.balance > 0 {
                    config.apy = config
                        .reward_rate
                        .mul(clpt_price)
                        .mul(86400u128)
                        .div(config.balance.mul(config.price));
                }
                if info.reward_per_token_paid == 0 {
                    info.reward_per_token_paid = new_per_token;
                }
                let new_reward = info
                    .balance
                    .mul(new_per_token.sub(info.reward_per_token_paid))
                    .div(MINING_MUL);
                info.reward = info.reward.add(new_reward);
                config.reward_per_token_stored = new_per_token;
                info.reward_per_token_paid = new_per_token;
                if is_add {
                    config.balance = config.balance.add(amount);
                } else {
                    config.balance = config.balance.sub(amount);
                }
                config.last_update = now;
            }
            None => {
                let config = AssetConfig {
                    reward_rate: 0,
                    reward_per_token_stored: 0,
                    last_update: now,
                    apy: 0,
                    price: 0,
                    balance: amount,
                };
                state.asset_config.insert(asset, config);
            }
        };
        if is_add {
            info.balance = info.balance.add(amount);
        } else {
            info.balance = info.balance.sub(amount);
        }
        state.users.insert(identify, info);
        Ok(())
    })
}

#[update]
async fn withdraw() -> Result<u128> {
    let amount = STATE.with(|s| {
        let mut state = s.borrow_mut();
        let now = api::time() / SECONDS;
        let mut amount = 0u128;
        let clpt_price = state.clpt_price.clone();
        for (_, config) in state.asset_config.iter_mut() {
            let new_per_token = _new_reward_per_token(config);
            if config.price > 0 && clpt_price > 0 && config.balance > 0 {
                config.apy = config
                    .reward_rate
                    .mul(clpt_price)
                    .mul(86400u128)
                    .div(config.balance.mul(config.price));
            }
            config.reward_per_token_stored = new_per_token;
            config.last_update = now;
        }
        let configs = state.asset_config.clone();
        let user = api::caller();
        for (account, config) in configs.iter() {
            let identify = UserAssetIdentify {
                user,
                underlying: account.clone(),
            };
            if config.reward_per_token_stored == 0 {
                continue;
            }
            let mut info = state.users.get(&identify).unwrap_or_default();
            let new_reward = info
                .balance
                .mul(
                    config
                        .reward_per_token_stored
                        .sub(info.reward_per_token_paid),
                )
                .div(MINING_MUL);
            amount = info.reward.add(new_reward).add(amount);
            info.reward = 0;
            info.reward_per_token_paid = config.reward_per_token_stored;
            state.users.insert(identify, info);
        }
        amount
    });
    let clpt = Token::new(STATE.with(|state| state.borrow().clpt).unwrap());
    clpt.transfer(TransferArgsICRC1 {
        amount: Nat::from(amount),
        created_at_time: None,
        fee: None,
        from_subaccount: None,
        memo: None,
        to: Account {
            owner: api::caller(),
            subaccount: None,
        },
    })
    .await
    .map_err(|_| Error::TransferFailure)?;
    Ok(amount)
}

#[query]
fn apr(account: Account) -> u128 {
    STATE.with(|s| {
        let state = s.borrow();
        let config = state.asset_config.get(&account).unwrap();
        if config.balance == 0 {
            return 0;
        }
        return config.reward_rate.div(config.balance);
    })
}

#[query]
fn balance(user: Principal) -> u128 {
    STATE.with(|s| {
        let state = s.borrow();
        let mut amount = 0u128;
        for (account, config) in state.asset_config.iter() {
            let identify = UserAssetIdentify {
                user,
                underlying: account.clone(),
            };
            let info = state.users.get(&identify).unwrap_or_default();
            let new_per_token = _new_reward_per_token(config);
            let new_reward = info
                .balance
                .mul(new_per_token.sub(info.reward_per_token_paid))
                .div(MINING_MUL);
            amount = amount.add(info.reward).add(new_reward);
        }
        amount
    })
}

pub fn check_caller() -> Result<(), String> {
    STATE.with(|state| {
        if state.borrow().caller.contains(&api::caller()) {
            Ok(())
        } else {
            Err("Unauthorized".to_string())
        }
    })
}
