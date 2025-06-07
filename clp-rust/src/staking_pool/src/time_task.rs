use crate::{Status, SECONDS, STATE};
use std::{
    borrow::BorrowMut,
    collections::HashMap,
    ops::{Div, Mul},
};

#[update(name = "collateralInspection")]
pub fn collateral_inspection() {
    ic_cdk::println!("collateral_inspection");
    let now = ic_cdk::api::time() / SECONDS;
    let today = now - now % 86400;
    STATE.with(|s| {
        let mut state = s.borrow_mut();
        let mut period_amounts: HashMap<u8, (u128, u128)> = HashMap::new();
        let mut users = HashMap::new();
        for (i, info) in state.clpt_users.iter() {
            if info.status == Status::Valid
                && today >= info.period_start
                && today <= info.period_end
            {
                let amount = period_amounts.get_mut(&info.period);
                match amount {
                    Some((x, _)) => {
                        *x = info.balance + *x;
                    }
                    None => {
                        period_amounts.insert(info.period, (info.balance, 0));
                    }
                }
                users.insert(i, info);
            }
        }
        let temp = 10u128.pow(8);
        let mut period_configs = state.period_config.clone();
        for (period, (amount, one_shared)) in period_amounts.iter_mut() {
            let config = period_configs.get_mut(&period).unwrap();
            *one_shared = config.daily_reward.mul(temp).div(*amount);
            config.apy = one_shared.mul(365).div(temp);
        }
        for (period, period_config) in state.period_config.borrow_mut().iter_mut() {
            let copy_config = period_configs.get(period).unwrap();
            if period_config.last_update == now {
                continue;
            }
            period_config.last_update = now;
            period_config.apy = copy_config.apy;
        }
        for (i, info) in users.into_iter() {
            let (_, one_shared) = period_amounts.get(&info.period).unwrap();
            let mut info = info;
            if info.last_update == now {
                continue;
            }
            info.earned += info.balance.mul(one_shared);
            if info.auto_investment && today == info.period_end {
                info.period_start = info.period_end;
                info.period_end = info.period_end + 86400u64.mul(info.period as u64);
                info.balance += info.earned;
                info.earned = 0;
            }
            info.last_update = now;
            state.clpt_users.insert(i, info);
        }
    });
}
