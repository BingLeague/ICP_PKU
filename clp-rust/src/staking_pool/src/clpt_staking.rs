use crate::token::{Token, TransferArgs, TransferArgsICRC1, TxError};
use crate::{with_user_lock, Error, Status, SECONDS};
use crate::{Account, DepositInfo, DepositPeriod, Result, UserPeriodIdentify, STATE};
use candid::Nat;
use ic_cdk::api::{self};
use std::ops::{Div, Mul};

#[update]
async fn stake(period: DepositPeriod, amount: u128, auto_investment: bool) -> Result<()> {
    let caller = ic_cdk::caller();
    with_user_lock(caller, || async {
        let clpt = STATE.with(|s| {
            let state = s.borrow();
            let period_config = state.period_config.get(&period).ok_or(Error::PeriodError)?;
            if amount < period_config.min_amount {
                return Err(Error::LessThanMinAmount);
            }
            state.clpt.clone().ok_or(Error::PeriodError)
        })?;
        let clpt = Token::new(clpt);
        clpt.transfer_from(TransferArgs {
            amount: Nat::from(amount),
            created_at_time: None,
            fee: None,
            from: Account {
                owner: api::caller(),
                subaccount: None,
            },
            memo: None,
            spender_subaccount: None,
            to: Account {
                owner: api::id(),
                subaccount: None,
            },
        })
        .await
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?;
        STATE.with(|s| {
            let caller = api::caller();
            let mut index = s.borrow().clpt_user_count.get(&caller).unwrap_or(0);
            let period_identity = UserPeriodIdentify {
                user: caller,
                index,
            };
            let now = api::time() / SECONDS;
            let period_start = now - now % 86400 + 86400;
            let period_end = period_start + period as u64 * 86400;
            let user = DepositInfo {
                balance: amount,
                created_at: now,
                period_start,
                period_end,
                period,
                last_update: now,
                auto_investment,
                ..Default::default()
            };
            s.borrow_mut().clpt_users.insert(period_identity, user);
            index += 1u128;
            s.borrow_mut().clpt_user_count.insert(caller, index);
            let mut state = s.borrow_mut();
            let period_config = state.period_config.get_mut(&period).unwrap();
            period_config.balance += amount;
            period_config.apy = period_config
                .daily_reward
                .div(period_config.balance)
                .mul(365);
            Ok(())
        })
    })
    .await
}

#[update]
async fn unstake(index: u128) -> Result<()> {
    let caller = ic_cdk::caller();
    with_user_lock(caller, || async {
        let now = api::time() / SECONDS;
        let (amount, period) = STATE.with(|s| {
            let period_identity = UserPeriodIdentify {
                user: api::caller(),
                index,
            };
            let user = s
                .borrow()
                .clpt_users
                .get(&period_identity)
                .unwrap_or_default();
            if user.status == Status::Valid {
                if now > user.period_end {
                    Ok((user.balance, user.period))
                } else {
                    Err(Error::InStaking)
                }
            } else {
                Err(Error::Invalid)
            }
        })?;
        let clpt = Token::new(STATE.with(|s| s.borrow().clpt.clone().unwrap()));
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
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?;
        STATE.with(|s| {
            let period_identity = UserPeriodIdentify {
                user: api::caller(),
                index,
            };
            let mut user = s.borrow().clpt_users.get(&period_identity).unwrap();
            user.status = Status::Invalid;
            let mut state = s.borrow_mut();
            state.clpt_users.insert(period_identity, user);
            let period_config = state.period_config.get_mut(&period).unwrap();
            period_config.balance -= amount;
            if period_config.balance > 0 {
                period_config.apy = period_config
                    .daily_reward
                    .div(period_config.balance)
                    .mul(365);
            }
            Ok(())
        })
    })
    .await
}

#[update(name = "autoInvestment")]
fn auto_investment(index: u128, auto: bool) -> Result<()> {
    let period_identity = UserPeriodIdentify {
        user: api::caller(),
        index,
    };
    STATE.with(|s| {
        let mut user = s.borrow().clpt_users.get(&period_identity).unwrap();
        if user.status == Status::Valid {
            user.auto_investment = auto;
            s.borrow_mut().clpt_users.insert(period_identity, user);
            Ok(())
        } else {
            Err(Error::Invalid)
        }
    })?;
    Ok(())
}
