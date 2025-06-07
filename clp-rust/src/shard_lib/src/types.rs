use candid::{CandidType, Nat, Principal};
use candid::{Decode, Encode};
use ic_stable_structures::{storable::Bound, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::result::Result as StdResult;
pub type Result<T = u128, E = Error> = StdResult<T, E>;
pub type OrderId = u32;
pub type Subaccount = [u8; 32];
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
    ZeroAddress,
    InsufficientBalance,
    AmountLessThanFee,
    InsufficientAllowance,
    InsufficientCollateral,
    TransferFailure,
    NotExists,
    Exists,
    Unauthorized,
    UnderlyingNotFound,
    ExceedingLoanAmount,
    UnderlyingNotEnable,
    BorrowLessThenCost,
    ExcessiveWithdrawal,
    PriceFormatError,
    Overflow,
    InvalidRate,
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
#[derive(
    Serialize, CandidType, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Debug,
)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

pub type Timestamp = u64;

#[derive(
    Serialize, CandidType, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Debug,
)]
pub struct UserAccountIdentify {
    pub underlying: Account,
    pub user: Principal,
}

#[derive(CandidType, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
pub struct UserAssetIdentify {
    pub underlying: Principal,
    pub user: Principal,
}

impl Storable for UserAssetIdentify {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}
