use crate::{Result, UserInfo, STATE};
use candid::{Nat, Principal};
use ic_cdk::api::{self};
use ic_cdk_macros::update;
use shard_lib::constant::SUB_ACCOUNT_STAKING_VAULT;
use shard_lib::token::*;
use shard_lib::types::*;
use shard_lib::utils::{create_subaccount, Staking};

/**
 * Vault Deposit
 */
#[update]
async fn deposit(underlying: Principal, amount: u128) -> Result<()> {
    let token = Token::new(underlying);
    STATE.with(|state| match state.borrow().underlying.get(&underlying) {
        Some(underlying_obj) => {
            if underlying_obj.enable {
                Ok(())
            } else {
                Err(Error::UnderlyingNotEnable)
            }
        }
        None => Err(Error::UnderlyingNotFound),
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
        .map_err(|e| match e {
            TxError::InsufficientAllowance { allowance: _ } => Error::InsufficientAllowance,
            TxError::InsufficientFunds { balance: _ } => Error::InsufficientBalance,
            _ => Error::TransferFailure,
        })?;
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.underlying.get_mut(&underlying).unwrap().deposit += amount;
        let identify = UserAssetIdentify {
            user: api::caller(),
            underlying,
        };
        match state.balances.get(&identify) {
            Some(mut info) => {
                info.balance += amount;
                state.balances.insert(identify, info);
            }
            None => {
                state.balances.insert(
                    identify,
                    UserInfo {
                        balance: amount,
                        borrow: 0,
                    },
                );
            }
        }
        Ok(())
    })
}
