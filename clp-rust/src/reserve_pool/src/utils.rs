use crate::types::Account;
use crate::Result;
use candid::Principal;

pub struct Staking {
    principal: Principal,
}
type TxReceipt = Result<()>;
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
