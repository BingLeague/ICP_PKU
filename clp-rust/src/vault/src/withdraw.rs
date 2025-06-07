use crate::{Result, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_cdk_macros::*;
use shard_lib::constant::{IS_SUB, SUB_ACCOUNT_STAKING_VAULT};
use shard_lib::token::*;
use shard_lib::types::*;
use shard_lib::utils::*;

#[update]
async fn withdraw(underlying: Principal, amount: u128) -> Result<()> {
    let token = Token::new(underlying);
    let identify = UserAssetIdentify {
        user: api::caller(),
        underlying,
    };
    let (need_fee, fee) = STATE.with(|state| {
        let state = state.borrow();
        let (need_fee, fee) = match state.underlying.get(&underlying) {
            Some(underlying_obj) => {
                if !underlying_obj.enable {
                    return Err(Error::UnderlyingNotEnable);
                }
                if amount < underlying_obj.need_fee + underlying_obj.fee {
                    return Err(Error::AmountLessThanFee);
                }
                if underlying_obj.deposit < amount {
                    Err(Error::InsufficientBalance)
                } else {
                    Ok((underlying_obj.need_fee, underlying_obj.fee))
                }
            }
            None => Err(Error::UnderlyingNotFound),
        }?;
        match state.balances.get(&identify) {
            Some(info) => {
                if info.balance < amount {
                    Err(Error::InsufficientBalance)
                } else {
                    Ok((need_fee, fee))
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
                subaccount: create_subaccount(SUB_ACCOUNT_STAKING_VAULT),
            },
            amount,
            IS_SUB,
        )
        .await
        .unwrap();
    ic_cdk::println!("withdraw amount {}", amount - need_fee);
    token
        .transfer(TransferArgsICRC1 {
            amount: Nat::from(amount - need_fee),
            created_at_time: None,
            fee: Some(Nat::from(fee)),
            from_subaccount: None,
            memo: None,
            to: Account {
                owner: api::caller(),
                subaccount: None,
            },
        })
        .await
        .map(|_| {
            STATE.with(|state| {
                let mut state = state.borrow_mut();
                let underlying_obj = state.underlying.get_mut(&underlying).unwrap();
                underlying_obj.deposit -= amount;
                underlying_obj.fee_balance += underlying_obj.need_fee - underlying_obj.fee;
                let mut user_info = state.balances.get(&identify).unwrap();
                user_info.balance -= amount;
                state.balances.insert(identify, user_info);
                Ok(())
            })
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance } => {
                ic_cdk::println!("balance: {} amount {}", balance, amount);
                Error::InsufficientBalance
            }
            _ => Error::TransferFailure,
        })?
}
