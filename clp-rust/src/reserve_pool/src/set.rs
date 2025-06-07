use crate::token::*;
use crate::{Error, Result, STATE};
use ic_cdk::api::{self};
use std::convert::TryInto;

#[update(name = "setFee", guard = "check_custodian")]
async fn set_fee(need_fee: u128) -> Result<()> {
    let fee: u128 = Token::new(STATE.with(|s| s.borrow().cusd.clone().unwrap()))
        .fee()
        .await
        .0
        .try_into()
        .unwrap();
    if need_fee < fee {
        return Err(Error::InvalidFee);
    }
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.fee = fee;
        state.need_fee = need_fee;
        Ok(())
    })
}

pub fn check_custodian() -> Result<(), String> {
    STATE.with(|state| {
        if state.borrow().custodians.contains(&api::caller()) {
            Ok(())
        } else {
            Err("Unauthorized".to_string())
        }
    })
}
