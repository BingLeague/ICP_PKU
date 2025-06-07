use crate::types::*;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_ledger_types::Memo;

pub type Tokens = Nat;
pub type Timestamp = u64;

pub struct Token {
    principal: Principal,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TxError {
    BadBurn { min_burn_amount: Tokens },
    BadFee { expected_fee: Tokens },
    CreatedInFuture { ledger_time: Timestamp },
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: Vec<u8> },
    InsufficientAllowance { allowance: Nat },
    InsufficientFunds { balance: Tokens },
    TemporarilyUnavailable,
    TooOld,
}
pub type TxReceipt = Result<Nat, TxError>;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgs {
    pub amount: Tokens,
    pub created_at_time: Option<Timestamp>,
    pub fee: Option<Tokens>,
    pub from: Account,
    pub memo: Option<Memo>,
    pub spender_subaccount: Option<Subaccount>,
    pub to: Account,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct AllownArgs {
    account: Account,
    spender: Account,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
struct Allowance {
    allowance: Nat,
    expires_at: Option<u64>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TransferArgsICRC1 {
    pub amount: Tokens,
    pub created_at_time: Option<Timestamp>,
    pub fee: Option<Tokens>,
    pub from_subaccount: Option<Subaccount>,
    pub memo: Option<Memo>,
    pub to: Account,
}

#[allow(non_snake_case)]
#[derive(CandidType, Clone, Debug, Deserialize)]
pub struct Metadata {
    pub logo: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub totalSupply: Nat,
    pub owner: Principal,
    pub fee: Nat,
}

impl Token {
    pub fn new(principal: Principal) -> Self {
        Token { principal }
    }

    pub async fn transfer(&self, args: TransferArgsICRC1) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            ic_cdk::api::call::call(self.principal, "icrc1_transfer", (args,)).await;
        call_result.unwrap().0
    }
    pub async fn transfer_from(&self, args: TransferArgs) -> TxReceipt {
        let call_result: Result<(TxReceipt,), _> =
            ic_cdk::api::call::call(self.principal, "icrc2_transfer_from", (args.clone(),)).await;
        call_result.unwrap().0
    }
}
