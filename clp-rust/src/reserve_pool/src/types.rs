use candid::{CandidType, Nat, Principal};

pub type OrderId = u32;
pub type Subaccount = Vec<u8>;
#[allow(non_snake_case)]
#[derive(CandidType, Clone)]
pub struct Order {
    pub id: OrderId,
    pub owner: Principal,
    pub from: Principal,
    pub fromAmount: Nat,
    pub to: Principal,
    pub toAmount: Nat,
}

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Error {
    Unauthorized,
    OnlyVault,
    InsufficientAllowance,
    TransferFailure,
    InsufficientBalance,
    AmountLessThanFee,
    ZeroAddress,
    Exists,
    InvalidFee,
}

pub type Tokens = Nat;

#[derive(CandidType, Deserialize)]
pub enum InterfaceId {
    Approval,
    TransactionHistory,
    Mint,
    Burn,
    TransferNotification,
}
#[derive(CandidType, Serialize, Debug, Deserialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

pub type Timestamp = u64;
