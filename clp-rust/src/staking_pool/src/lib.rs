#![allow(clippy::collapsible_else_if)]

#[macro_use]
extern crate ic_cdk_macros;
#[macro_use]
extern crate serde;

use candid::{CandidType, Principal};
use candid::{Decode, Encode};
use ic_cdk::api::{self};
use ic_stable_structures::{
    storable::Bound, writer::Writer, Memory as _, StableBTreeMap, Storable,
};
use memory::Memory;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::iter::FromIterator;
use std::result::Result as StdResult;

use types::*;
mod clpt_staking;
mod memory;
pub mod mining;
mod query;
mod set;
mod time_task;
mod token;
pub mod types;
mod utils;

pub const SECONDS: u64 = 1_000_000_000;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
    static GUARD: RefCell<Guard> = RefCell::new(Guard::default());
}

#[derive(Default)]
struct Guard {
    // ... 其他字段 ...
    user_locks: HashSet<Principal>,
}

impl Guard {
    fn acquire_lock(&mut self, user: Principal) -> Result<()> {
        if !self.user_locks.insert(user) {
            Err(Error::UserLocked)
        } else {
            Ok(())
        }
    }
    fn release_lock(&mut self, user: Principal) {
        self.user_locks.remove(&user);
    }
}

async fn with_user_lock<F, Fut, R>(user: Principal, f: F) -> Result<R>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<R>>,
{
    // 获取锁
    GUARD.with(|s| s.borrow_mut().acquire_lock(user))?;

    // 执行异步函数
    let result = f().await;

    // 无论函数执行成功与否，都释放锁
    GUARD.with(|s| s.borrow_mut().release_lock(user));

    result
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct AssetInfo {
    balance: u128,
    reward: u128,
    withdrawn: u128,
    reward_per_token_paid: u128,
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct AssetConfig {
    reward_rate: u128,
    reward_per_token_stored: u128,
    last_update: u64,
    price: u128,
    balance: u128,
    apy: u128,
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct PeriodConfig {
    daily_reward: u128,
    balance: u128,
    min_amount: u128,
    last_update: u64,
    apy: u128,
}

#[derive(CandidType, Deserialize, Serialize, PartialEq, Clone)]
enum Status {
    Valid,
    Invalid,
}

impl Default for Status {
    fn default() -> Self {
        Self::Valid
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct DepositInfo {
    status: Status,
    period: DepositPeriod,
    last_update: u64,
    created_at: u64,
    period_start: u64,
    period_end: u64,
    balance: u128,
    earned: u128,
    auto_investment: bool,
}

#[derive(CandidType, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
struct UserAssetIdentify {
    underlying: Account,
    user: Principal,
}

#[derive(CandidType, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
struct UserPeriodIdentify {
    user: Principal,
    index: u128,
}

impl Storable for AssetInfo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

impl Storable for DepositInfo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
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

impl Storable for UserPeriodIdentify {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

type DepositPeriod = u8;

#[derive(Serialize, Deserialize)]
struct State {
    custodians: HashSet<Principal>,
    caller: HashSet<Principal>,
    clpt: Option<Principal>,
    clpt_price: u128,
    asset_config: HashMap<Account, AssetConfig>,
    period_config: HashMap<DepositPeriod, PeriodConfig>,
    #[serde(skip, default = "init_stable_data")]
    users: StableBTreeMap<UserAssetIdentify, AssetInfo, Memory>,
    #[serde(skip, default = "init_stable_data_clpt")]
    clpt_users: StableBTreeMap<UserPeriodIdentify, DepositInfo, Memory>,
    #[serde(skip, default = "init_stable_data_clpt_count")]
    clpt_user_count: StableBTreeMap<Principal, u128, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            custodians: HashSet::new(),
            clpt: None,
            clpt_price: 0,
            caller: HashSet::new(),
            asset_config: HashMap::new(),
            period_config: HashMap::new(),
            users: init_stable_data(),
            clpt_users: init_stable_data_clpt(),
            clpt_user_count: init_stable_data_clpt_count(),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType)]
struct InitArgs {
    timer_interval_secs: u64,
}

#[init]
fn init(args: InitArgs) {
    let interval = std::time::Duration::from_secs(args.timer_interval_secs);
    ic_cdk::println!("Starting a periodic task with interval {interval:?}");
    ic_cdk_timers::set_timer_interval(interval, || time_task::collateral_inspection());
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.custodians = HashSet::from_iter([api::caller()]);
    });
}

type Result<T = u128, E = Error> = StdResult<T, E>;

#[pre_upgrade]
fn pre_upgrade() {
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    STATE
        .with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes))
        .expect("failed to encode state");

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

// A post-upgrade hook for deserializing the data back into the heap.
#[post_upgrade]
fn post_upgrade() {
    let memory = memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    STATE.with(|s| *s.borrow_mut() = state);
}

fn init_stable_data() -> StableBTreeMap<UserAssetIdentify, AssetInfo, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

fn init_stable_data_clpt() -> StableBTreeMap<UserPeriodIdentify, DepositInfo, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory_clpt())
}

fn init_stable_data_clpt_count() -> StableBTreeMap<Principal, u128, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory_clpt_count())
}

ic_cdk::export_candid!();
