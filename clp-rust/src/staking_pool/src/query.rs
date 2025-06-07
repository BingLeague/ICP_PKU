use std::borrow::Borrow;

use crate::{
    Account, AssetConfig, AssetInfo, DepositInfo, DepositPeriod, PeriodConfig, UserAssetIdentify,
    UserPeriodIdentify, STATE,
};
use candid::Principal;

#[query(name = "assetConfig")]
fn asset_config() -> Vec<(Account, AssetConfig)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .asset_config
            .borrow()
            .iter()
            .map(|(account, asset_config)| (account.clone(), asset_config.clone()))
            .collect()
    })
}

#[query(name = "periodConfig")]
fn period_config() -> Vec<(DepositPeriod, PeriodConfig)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .period_config
            .borrow()
            .iter()
            .map(|(account, asset_config)| (account.clone(), asset_config.clone()))
            .collect()
    })
}

#[query]
fn balances(user: UserAssetIdentify) -> AssetInfo {
    STATE.with(|state| {
        let state = state.borrow();
        state.users.get(&user).unwrap_or_default()
    })
}

#[query(name = "allUserBalances")]
fn all_user_balance() -> Vec<(UserAssetIdentify, AssetInfo)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .users
            .iter()
            .map(|(addr, user_asset)| (addr, user_asset))
            .collect()
    })
}

#[query(name = "DepositInfo")]
fn deposite_info(user: UserPeriodIdentify) -> DepositInfo {
    STATE.with(|state| {
        let state = state.borrow();
        state.clpt_users.get(&user).unwrap_or_default()
    })
}

#[query(name = "clptBalances")]
fn clpt_balances(user: Principal) -> Vec<(u128, DepositInfo)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .clpt_users
            .iter()
            .filter(|(identity, _)| identity.user == user)
            .map(|(identity, user_asset)| (identity.index, user_asset.clone())) // 返回符合条件的数据
            .collect()
    })
}

#[query(name = "allClptBalances")]
fn clpt_all_user_balance() -> Vec<(UserPeriodIdentify, DepositInfo)> {
    STATE.with(|state| {
        let state = state.borrow();
        state
            .clpt_users
            .iter()
            .map(|(addr, user_asset)| (addr, user_asset))
            .collect()
    })
}
