use crate::{UnderlyingInfo, STATE};
use candid::Principal;
use ic_cdk::api::{self};
use ic_cdk_macros::*;
use shard_lib::permission::Role;
use shard_lib::token::*;
use shard_lib::types::*;
use std::convert::TryInto;

#[update(name = "setCUSD", guard = "check_custodian")]
async fn set_cusd(cusd: Principal) -> Result<()> {
    if cusd == Principal::from_slice(&[]) {
        return Err(Error::ZeroAddress);
    }
    let decimals = Token::new(cusd).decimals().await;
    if decimals != 8 {
        return Err(Error::ZeroAddress);
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if None == state.cusd {
            state.cusd = Some(cusd);
            Ok(())
        } else {
            Err(Error::Exists)
        }
    })
}

#[update(name = "setReservePool", guard = "check_custodian")]
async fn set_reserve_pool(pool: Principal) -> Result<()> {
    if pool == Principal::from_slice(&[]) {
        return Err(Error::ZeroAddress);
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        if None == state.reserve_pool {
            state.reserve_pool = Some(pool);
            Ok(())
        } else {
            Err(Error::Exists)
        }
    })
}

#[update(name = "setMockPrice", guard = "check_custodian")]
async fn set_mock_price(underlying: Principal, scaled_price: u128) -> Result<()> {
    STATE.with(
        |state| match state.borrow_mut().underlying.get_mut(&underlying) {
            Some(underlying_info) => {
                underlying_info.price = scaled_price;
                Ok(())
            }
            None => Err(Error::UnderlyingNotFound),
        },
    )
}

#[update(name = "addUnderlying", guard = "check_custodian")]
async fn add_underlying(
    token: Principal,
    allow: bool,
    need_fee: u128,
    cost: u128,
    name: String,
    liquidate_rate: u128,
) -> Result<()> {
    let decimals = Token::new(token).decimals().await;
    let fee: u128 = Token::new(token).fee().await.0.try_into().unwrap();
    if need_fee < fee {
        return Err(Error::InvalidFee);
    }
    if liquidate_rate > 100 {
        return Err(Error::InvalidRate);
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        match state.underlying.get_mut(&token) {
            Some(underlying_info) => {
                underlying_info.enable = allow;
                underlying_info.fee = fee;
                underlying_info.liquidate_rate = liquidate_rate;
                underlying_info.need_fee = need_fee;
            }
            None => {
                state.underlying.insert(
                    token,
                    UnderlyingInfo {
                        enable: allow,
                        decimals,
                        name,
                        fee,
                        need_fee,
                        liquidate_rate,
                        cost,
                        ..Default::default()
                    },
                );
            }
        }
        Ok(())
    })
}

pub fn check_custodian() -> Result<(), std::string::String> {
    STATE.with(|state| {
        let state = state.borrow();
        let caller = api::caller();
        if let Some(permission) = state.permission_state.permissions.get(&caller) {
            if permission.role == Role::Owner || permission.role == Role::Admin {
                Ok(())
            } else {
                Err("Caller is not a custodian".into())
            }
        } else {
            Err("Caller has no permissions".into())
        }
    })
}
