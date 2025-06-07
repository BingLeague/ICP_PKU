use crate::token::*;
use crate::types::Account;
use crate::utils::Staking;
use crate::{Error, Result, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};

#[update(name = "withdrawUnderlying")]
async fn withdraw_underlying(underlying: Principal, amount: u128) -> Result<()> {
    STATE.with(|state| {
        let state = state.borrow();
        match state.users.get(&api::caller()) {
            Some(info) => match info.underlies.get(&underlying) {
                Some(user_asset) => {
                    if amount > user_asset.wait_withdrawal {
                        Err(Error::InsufficientBalance)
                    } else {
                        Ok(())
                    }
                }
                None => Err(Error::InsufficientBalance),
            },
            None => Err(Error::InsufficientBalance),
        }
    })?;
    let token = Token::new(underlying);
    token
        .transfer(TransferArgsICRC1 {
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
        .map(|_| {
            STATE.with(|state| {
                state
                    .borrow_mut()
                    .users
                    .get(&api::caller())
                    .unwrap()
                    .underlies
                    .get_mut(&underlying)
                    .map(|user_asset| {
                        user_asset.wait_withdrawal -= amount;
                    })
                    .unwrap();
            });
            Ok(())
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?
}

#[update]
async fn withdraw(amount: u128) -> Result<()> {
    let (cusd, need_fee, fee) = STATE.with(|state| {
        let state = state.borrow();
        (state.cusd.unwrap(), state.need_fee, state.fee)
    });
    if amount < need_fee + fee {
        return Err(Error::AmountLessThanFee);
    }
    let staking = Staking::new(STATE.with(|state| state.borrow().staking_pool).unwrap());
    staking
        .update_balance(
            api::caller(),
            Account {
                owner: cusd,
                subaccount: None,
            },
            amount,
            false,
        )
        .await
        .unwrap();
    let token = Token::new(cusd);
    STATE.with(|state| {
        let state = state.borrow();
        if state.balance < amount {
            return Err(Error::InsufficientBalance);
        }
        match state.users.get(&api::caller()) {
            Some(info) => {
                if info.balance < amount {
                    Err(Error::InsufficientBalance)
                } else {
                    Ok(())
                }
            }
            None => Err(Error::InsufficientBalance),
        }
    })?;
    token
        .transfer(TransferArgsICRC1 {
            amount: Nat::from(amount - need_fee),
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
                let mut state = state.borrow_mut();
                state.balance -= amount;
                let mut info = state.users.get(&api::caller()).unwrap();
                info.balance -= amount;
                state.users.insert(api::caller(), info)
            });
            Ok(())
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?
}
