use crate::{UserInfo, STATE};
use candid::Principal;

#[query(name = "userUnderlyingBalance")]
async fn user_underlying_balances(user: Principal, underlying: Principal) -> u128 {
    STATE.with(|state| match state.borrow().users.get(&user) {
        Some(user) => user
            .underlies
            .get(&underlying)
            .map(|user_asset| user_asset.wait_withdrawal)
            .unwrap_or(0),
        None => 0,
    })
}

#[query(name = "userUnderlyingBalances")]
async fn user_underlying_balance(user: Principal) -> Vec<(Principal, u128)> {
    STATE.with(|state| match state.borrow().users.get(&user) {
        Some(user) => user
            .underlies
            .iter()
            .map(|(underlying, info)| (underlying.clone(), info.wait_withdrawal))
            .collect(),
        None => {
            vec![]
        }
    })
}

#[query(name = "userBalance")]
async fn user_balance(user: Principal) -> u128 {
    STATE.with(|state| {
        state
            .borrow()
            .users
            .get(&user)
            .map(|info| info.balance)
            .unwrap_or(0)
    })
}

#[query(name = "allUsers")]
async fn all_users() -> Vec<(Principal, UserInfo)> {
    STATE.with(|state| {
        state
            .borrow()
            .users
            .iter()
            .map(|(user, info)| (user, info))
            .collect()
    })
}

#[query(name = "liquidateUnderlying")]
async fn debet_underlying(underlying: Principal) -> u128 {
    STATE.with(|state| {
        state
            .borrow()
            .liquidate_underlying
            .get(&underlying)
            .map(|info| *info)
            .unwrap_or(0)
    })
}

#[query(name = "userInfo")]
fn user_info(user: Principal) -> UserInfo {
    STATE.with(|state| {
        if let Some(user_info) = state.borrow().users.get(&user) {
            user_info.clone()
        } else {
            UserInfo::default()
        }
    })
}

#[query(name = "stats")]
fn stats() -> (u128, u128) {
    STATE.with(|state| {
        let state = state.borrow();
        (state.balance, state.total_sub)
    })
}
