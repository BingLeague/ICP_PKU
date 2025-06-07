use crate::Account;
use candid::Principal;

use crate::token::TxReceipt;
use crate::types::Result;

pub struct Staking {
    principal: Principal,
}

impl Staking {
    pub fn new(principal: Principal) -> Self {
        Staking { principal }
    }

    pub async fn update_balance(
        &self,
        user: Principal,
        account: Account,
        amount: u128,
        is_add: bool,
    ) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> = ic_cdk::api::call::call(
            self.principal,
            "update_balance",
            (user, account, amount, is_add),
        )
        .await;
        call_result.unwrap().0
    }
}

/// Creates a subaccount array with a specified first byte
///
/// # Arguments
///
/// * `first_byte` - The value to set as the first byte of the 32-byte array
///
/// # Returns
///
/// A 32-byte array with the first byte set to the specified value and all other bytes set to 0
pub fn create_subaccount(first_byte: u8) -> Option<[u8; 32]> {
    let mut array = [0u8; 32];
    array[0] = first_byte;
    Some(array)
}
