use crate::token::*;
use crate::types::Account;
use crate::types::Error;
use crate::utils::Staking;
use crate::{Result, UserInfo, STATE};
use candid::Nat;
use ic_cdk::api::{self};

#[update]
async fn deposit(amount: u128) -> Result<()> {
    let staking = Staking::new(STATE.with(|state| state.borrow().staking_pool).unwrap());
    let token = Token::new(STATE.with(|state| state.borrow().cusd.unwrap()));
    staking
        .update_balance(
            api::caller(),
            Account {
                owner: token.principal,
                subaccount: None,
            },
            amount,
            true,
        )
        .await
        .unwrap();
    token
        .transfer_from(TransferArgs {
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
                state.balance += amount;
                match state.users.get(&api::caller()) {
                    Some(mut user_info) => {
                        user_info.balance += amount;
                        state.users.insert(api::caller(), user_info);
                    }
                    None => {
                        state.users.insert(
                            api::caller(),
                            UserInfo {
                                balance: amount,
                                ..Default::default()
                            },
                        );
                    }
                }
                Ok(())
            })
        })
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?
}
