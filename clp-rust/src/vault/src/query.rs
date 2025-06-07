use crate::time_task::calculate_liquidate_amount;
use crate::{Result, UnderlyingInfo, UserInfo, STATE};
use candid::Principal;
use ic_cdk_macros::*;
use shard_lib::constant::SUB_ACCOUNT_PLATFORM_INCOME;
use shard_lib::permission::PermissionState;
use shard_lib::types::*;
use shard_lib::utils::create_subaccount;

#[query(name = "getCustodian")]
fn get_permission() -> PermissionState {
    STATE.with(|state| state.borrow().permission_state.clone())
}

#[query]
fn underlying(token: Principal) -> Result<UnderlyingInfo, Error> {
    STATE.with(|state| match state.borrow().underlying.get(&token) {
        Some(info) => Ok(info.clone()),
        None => Err(Error::NotExists),
    })
}

#[query(name = "underlyingList")]
fn underlying_list() -> Vec<(Principal, UnderlyingInfo)> {
    STATE.with(|state| state.borrow().underlying.clone().into_iter().collect())
}

#[query]
fn balance(user: Principal, underlying: Principal) -> UserInfo {
    STATE.with(|state| {
        state
            .borrow()
            .balances
            .get(&UserAssetIdentify { user, underlying })
            .unwrap_or_default()
    })
}

#[query]
fn balances(user: Principal) -> Vec<(Principal, UserInfo)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .underlying
            .iter()
            .map(|(underlying, _)| {
                (
                    underlying.clone(),
                    state
                        .balances
                        .get(&UserAssetIdentify {
                            user,
                            underlying: underlying.clone(),
                        })
                        .unwrap_or_default(),
                )
            })
            .collect()
    })
}

#[query(name = "allBalances")]
fn all_balance() -> Vec<(UserAssetIdentify, UserInfo)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .balances
            .iter()
            .map(|(addr, user_asset)| (addr, user_asset))
            .collect()
    })
}

#[query(name = "maxBorrow")]
async fn max_borrow(user: Principal, underlying: Principal) -> u128 {
    STATE.with(|state| {
        let state = state.borrow();
        let underlying_info = state.underlying.get(&underlying).clone().unwrap();
        match state.balances.get(&UserAssetIdentify { user, underlying }) {
            Some(info) => {
                let max = calculate_liquidate_amount(
                    info.balance,
                    underlying_info.price,
                    underlying_info.liquidate_rate,
                    underlying_info.decimals,
                )
                .unwrap();
                max
            }
            None => 0u128,
        }
    })
}

#[query(name = "excessive")]
fn excessive_cusd() -> u128 {
    STATE.with(|state| state.borrow().excessive_cusd)
}

#[query(name = "account")]
fn account() -> Account {
    Account {
        owner: ic_cdk::id(),
        subaccount: create_subaccount(SUB_ACCOUNT_PLATFORM_INCOME),
    }
}

#[query(name = "feeBalance")]
fn fee_balance() -> u128 {
    STATE.with(|state| state.borrow().fee_balance)
}

#[query(name = "withdrawnCusdFee")]
fn withdrawn_cusd_fee() -> u128 {
    STATE.with(|state| state.borrow().withdrawn_cusd_fee)
}

// #[query(name = "getState")]
// fn get_state() -> State {
//     STATE.with(|state| mem::take(&mut *state.borrow_mut()))
// }
