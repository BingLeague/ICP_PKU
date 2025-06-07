use crate::STATE;
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_cdk_macros::*;
use shard_lib::constant::*;
use shard_lib::token::*;
use shard_lib::types::*;
use shard_lib::utils::*;

/**
 * vault repayment
 */
#[update]
async fn repayment(underlying: Principal, amount: u128) -> Result<(), Error> {
    let identify = UserAssetIdentify {
        user: api::caller(),
        underlying,
    };
    STATE.with(|state| {
        let state = state.borrow();
        match state.underlying.get(&underlying) {
            Some(underlying_obj) => {
                if !underlying_obj.enable {
                    return Err(Error::UnderlyingNotEnable);
                }
                if amount > underlying_obj.borrow {
                    Err(Error::ExceedingLoanAmount)
                } else {
                    Ok(())
                }
            }
            None => Err(Error::UnderlyingNotFound),
        }?;
        match state.balances.get(&identify) {
            Some(info) => {
                if amount > info.borrow {
                    Err(Error::ExceedingLoanAmount)
                } else {
                    Ok(())
                }
            }
            None => Err(Error::InsufficientBalance),
        }
    })?;
    let cusd = Token::new(STATE.with(|state| state.borrow().cusd).unwrap());
    cusd.transfer_from(TransferArgs {
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
    .map(|_| {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            let underlying_obj = state.underlying.get_mut(&underlying).unwrap();
            underlying_obj.borrow -= amount;
            let mut user_info = state.balances.get(&identify).unwrap();
            user_info.borrow -= amount;
            state.balances.insert(identify, user_info);
            // Ok(())
        })
    })
    .map_err(|e| match e {
        TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
        TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
        _ => Error::TransferFailure,
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
            IS_SUB,
        )
        .await
        .unwrap();
    Ok(())
}
