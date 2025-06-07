use candid::{CandidType, Principal};
pub type Subaccount = Vec<u8>;

#[derive(CandidType, Deserialize, Debug)]
pub enum Error {
    UserLocked,
    OnlyVault,
    InsufficientAllowance,
    InsufficientBalance,
    Invalid,
    InStaking,
    TransferFailure,
    ZeroAddress,
    PeriodError,
    LessThanMinAmount,
    BalanceIsZero,
    AssetRewardRateZero,
}
#[derive(
    CandidType, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq, Hash, Debug,
)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}
