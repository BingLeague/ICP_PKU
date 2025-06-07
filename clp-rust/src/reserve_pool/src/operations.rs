use crate::set::check_custodian;
use crate::token::*;
use crate::types::Account;
use crate::{Error, Result, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use std::borrow::Borrow;
use std::convert::TryInto;

#[update(name = "withdrawLiquidateUnderlying", guard = "check_custodian")]
async fn withdraw_debet_underlying(underlying: Principal, amount: u128) -> Result<()> {
    STATE.with(|state| {
        let state = state.borrow();
        let liquidate_underlying = state.liquidate_underlying.borrow();
        match liquidate_underlying.get(&underlying) {
            Some(balance) => {
                if amount > *balance {
                    Err(Error::InsufficientBalance)
                } else {
                    Ok(())
                }
            }
            None => Err(Error::InsufficientBalance),
        }
    })?;
    let token = Token::new(underlying);
    let fee: u128 = token.fee().await.0.try_into().unwrap();
    if amount < fee * 2 {
        return Err(Error::AmountLessThanFee);
    }
    token
        .transfer(TransferArgsICRC1 {
            amount: Nat::from(amount - fee),
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
        .map(|_| {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .liquidate_underlying
                    .get_mut(&underlying)
                    .map(|balance| {
                        *balance -= amount;
                        Ok(())
                    })
                    .unwrap()
            })
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?
}

// #[update(name = "fixBalance", guard = "check_custodian")]
// async fn fix_balance(balance: u128) -> Result<()> {
//     STATE.with(|state| {
//         let mut state = state.borrow_mut();
//         state.balance = balance;
//         Ok(())
//     })
// }
