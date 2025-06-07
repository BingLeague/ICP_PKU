use crate::mining::MINING_MUL;
use crate::SECONDS;
use crate::{Account, AssetConfig, DepositPeriod, PeriodConfig, Result, STATE};
use candid::Principal;
use ic_cdk::api::{self};
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Add, Div, Mul, Sub};

#[update(name = "setTokenPrice", guard = "check_custodian")]
fn set_token_price(account: Account, price: u128) -> Result<()> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let clpt_price = state.clpt_price.clone();
        match state.asset_config.get_mut(&account) {
            Some(config) => {
                config.price = price;
                let new_per_token = _new_reward_per_token(config);
                if config.price > 0 && clpt_price > 0 && config.balance > 0 {
                    config.apy = config
                        .reward_rate
                        .mul(clpt_price)
                        .mul(86400u128)
                        .div(config.balance.mul(config.price));
                }
                config.reward_per_token_stored = new_per_token;
                config.last_update = api::time() / SECONDS;
            }
            None => {
                state.asset_config.insert(
                    account,
                    AssetConfig {
                        reward_rate: 0,
                        reward_per_token_stored: 0,
                        last_update: api::time() / SECONDS,
                        balance: 0,
                        apy: 0,
                        price,
                    },
                );
            }
        }
    });
    Ok(())
}

#[update(name = "setAssetConfig", guard = "check_custodian")]
fn set_asset_config(account: Account, rate: u128, price: u128) -> Result<()> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let clpt_price = state.clpt_price.clone();
        match state.asset_config.get_mut(&account) {
            Some(config) => {
                config.price = price;
                let new_per_token = _new_reward_per_token(config);
                if config.price > 0 && clpt_price > 0 && config.balance > 0 {
                    config.apy = config
                        .reward_rate
                        .mul(clpt_price)
                        .mul(86400u128)
                        .div(config.balance.mul(config.price));
                }
                config.reward_per_token_stored = new_per_token;
                config.last_update = api::time() / SECONDS;
                config.reward_rate = rate;
            }
            None => {
                state.asset_config.insert(
                    account,
                    AssetConfig {
                        reward_rate: rate,
                        reward_per_token_stored: 0,
                        last_update: api::time() / SECONDS,
                        balance: 0,
                        apy: 0,
                        price,
                    },
                );
            }
        }
    });
    Ok(())
}

#[update(name = "setPeriodConfig", guard = "check_custodian")]
fn set_period_config(period: DepositPeriod, daily_reward: u128, min_amount: u128) -> Result<()> {
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        match state.period_config.get_mut(&period) {
            Some(config) => {
                config.daily_reward = daily_reward;
                config.min_amount = min_amount;
            }
            None => {
                state.period_config.insert(
                    period,
                    PeriodConfig {
                        apy: 0,
                        daily_reward,
                        balance: 0,
                        min_amount,
                        last_update: api::time() / SECONDS,
                    },
                );
            }
        }
    });
    Ok(())
}

#[update(guard = "check_custodian", name = "setClptPrice")]
fn set_clpt_price(price: u128) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.clpt_price = price;
    });
}

pub fn _new_reward_per_token(config: &AssetConfig) -> u128 {
    if config.balance == 0 {
        return config.reward_per_token_stored;
    }
    let distance = api::time().div(SECONDS).sub(config.last_update) as u128;
    return distance
        .mul(config.reward_rate)
        .mul(MINING_MUL)
        .div(config.balance)
        .add(config.reward_per_token_stored);
}

#[update(guard = "check_custodian", name = "paramInit")]
fn param_init(vault: Principal, clpt: Principal, reserve_pool: Principal) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.clpt = Some(clpt);
        state.caller = HashSet::from_iter([vault, reserve_pool, api::caller()]);
    });
}

pub fn check_custodian() -> Result<(), String> {
    STATE.with(|state| {
        if state.borrow().custodians.contains(&api::caller()) {
            Ok(())
        } else {
            Err("Unauthorized".to_string())
        }
    })
}
