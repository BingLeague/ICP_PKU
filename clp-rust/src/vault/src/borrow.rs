use crate::time_task::calculate_liquidate_amount;
use crate::{Result, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_cdk_macros::update;
use shard_lib::constant::{IS_ADD, SUB_ACCOUNT_PLATFORM_INCOME, SUB_ACCOUNT_STAKING_BORROW};
use shard_lib::token::*;
use shard_lib::types::*;
use shard_lib::utils::{create_subaccount, Staking};

#[update]
async fn borrow(underlying: Principal, amount: u128) -> Result<()> {
    let cost = STATE.with(|state| {
        let state = state.borrow();
        let (decimals, price, cost, liquidate_rate) = match state.underlying.get(&underlying) {
            Some(underlying_obj) => {
                if !underlying_obj.enable {
                    return Err(Error::UnderlyingNotEnable);
                }
                if amount < underlying_obj.cost {
                    return Err(Error::BorrowLessThenCost);
                }
                Ok((
                    underlying_obj.decimals,
                    underlying_obj.price,
                    underlying_obj.cost,
                    underlying_obj.liquidate_rate,
                ))
            }
            None => Err(Error::UnderlyingNotFound),
        }?;
        match state.balances.get(&UserAssetIdentify {
            user: api::caller(),
            underlying,
        }) {
            Some(info) => {
                let total_borrow = amount.checked_add(info.borrow).ok_or(Error::Overflow)?;
                let liquidate_amount =
                    calculate_liquidate_amount(info.balance, price, liquidate_rate, decimals)?;
                if liquidate_amount < total_borrow {
                    Err(Error::InsufficientCollateral)
                } else {
                    Ok(cost)
                }
            }
            None => Err(Error::InsufficientBalance),
        }
    })?;
    let staking = Staking::new(STATE.with(|state| state.borrow().staking_pool).unwrap());
    staking
        .update_balance(
            api::caller(),
            Account {
                owner: underlying.clone(),
                subaccount: create_subaccount(SUB_ACCOUNT_STAKING_BORROW),
            },
            amount,
            IS_ADD,
        )
        .await
        .unwrap();
    let cusd = Token::new(STATE.with(|state| state.borrow().cusd).unwrap());
    cusd.transfer(TransferArgsICRC1 {
        amount: Nat::from(amount - cost),
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
    cusd.transfer(TransferArgsICRC1 {
        amount: Nat::from(cost),
        created_at_time: None,
        fee: None,
        from_subaccount: None,
        memo: None,
        to: Account {
            owner: api::id(),
            subaccount: create_subaccount(SUB_ACCOUNT_PLATFORM_INCOME),
        },
    })
    .await
    .map(|_| {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.fee_balance += cost;
            let underlying_obj = state.underlying.get_mut(&underlying).unwrap();
            underlying_obj.borrow += amount;
            let identify = UserAssetIdentify {
                user: api::caller(),
                underlying,
            };
            let mut user_info = state.balances.get(&identify).unwrap();
            user_info.borrow += amount;
            state.balances.insert(identify, user_info);
            Ok(())
        })
    })
    .map_err(|e| match e {
        TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
        TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailure,
    })?
}
