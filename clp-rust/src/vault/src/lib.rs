#![allow(clippy::collapsible_else_if)]

use candid::{CandidType, Principal};
use candid::{Decode, Encode};
use ic_cdk::api::{self};
use ic_cdk_macros::*;
use ic_stable_structures::{
    storable::Bound, writer::Writer, Memory as _, StableBTreeMap, Storable,
};
use serde::{Deserialize, Serialize};
use shard_lib::permission::PermissionState;
use shard_lib::{types::*, Permission, Role, SECONDS};
use std::collections::{HashMap, HashSet};
use std::{borrow::Cow, cell::RefCell};

mod borrow;
mod deposit;
mod operations;
mod query;
mod repayment;
mod set;
mod time_task;
mod withdraw;
mod xrc;

mod memory;
use memory::Memory;

use shard_lib::permission::*;
use shard_lib::types::Result;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(Serialize, Deserialize, Clone, CandidType, Default, Debug)]
struct UnderlyingInfo {
    name: String,
    decimals: u8,
    deposit: u128, //underlying asset
    borrow: u128,  //cusd
    fee: u128,
    need_fee: u128,
    price: u128,
    cost: u128,
    fee_balance: u128,
    liquidate_rate: u128,
    enable: bool,
}

#[derive(CandidType, Deserialize, Default, Clone, Copy)]
struct UserInfo {
    balance: u128, //underlying asset
    borrow: u128,  //cusd
}

impl Storable for UserInfo {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(CandidType, Deserialize)]
struct UserAsset {
    asset: HashMap<Principal, UserInfo>,
}

impl Storable for UserAsset {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }
    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
    const BOUND: Bound = Bound::Unbounded;
}

#[derive(Serialize, Deserialize)]
struct State {
    underlying: HashMap<Principal, UnderlyingInfo>,
    cusd: Option<Principal>,
    timer_interval_secs: u64,
    reserve_pool: Option<Principal>,
    staking_pool: Option<Principal>,
    fee_balance: u128,
    excessive_cusd: u128,
    withdrawn_cusd_fee: u128,
    permission_state: PermissionState,
    #[serde(skip, default = "init_stable_data")]
    balances: StableBTreeMap<UserAssetIdentify, UserInfo, Memory>,
}

// A pre-upgrade hook for serializing the data stored on the heap.
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
#[ic_cdk_macros::post_upgrade]
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

fn init_stable_data() -> StableBTreeMap<UserAssetIdentify, UserInfo, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

impl Default for State {
    fn default() -> Self {
        Self {
            underlying: HashMap::new(),
            cusd: None,
            timer_interval_secs: 60,
            reserve_pool: None,
            staking_pool: None,
            fee_balance: 0,
            excessive_cusd: 0,
            withdrawn_cusd_fee: 0,
            permission_state: PermissionState::default(),
            balances: init_stable_data(),
        }
    }
}

#[derive(Serialize, Deserialize, CandidType)]
struct InitArgs {
    timer_interval_secs: u64,
    staking_pool: Principal,
    owner: Principal,
    admin: Principal,
}

#[init]
fn init(args: InitArgs) {
    let interval = std::time::Duration::from_secs(args.timer_interval_secs);
    ic_cdk::println!("Starting a periodic task with interval {interval:?}");
    ic_cdk_timers::set_timer_interval(interval, || {
        ic_cdk::spawn(time_task::collateral_inspection())
    });
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        let admin_permission = Permission {
            role: Role::Admin,
            allowed_functions: HashSet::new(),
            created_at: api::time() / SECONDS,
            last_modified: api::time() / SECONDS,
        };
        let owner_permission = Permission {
            role: Role::Owner,
            allowed_functions: HashSet::new(),
            created_at: api::time() / SECONDS,
            last_modified: api::time() / SECONDS,
        };
        state
            .permission_state
            .permissions
            .insert(args.admin, admin_permission);
        state
            .permission_state
            .permissions
            .insert(args.owner, owner_permission);
        state.timer_interval_secs = args.timer_interval_secs;
        state.staking_pool = Some(args.staking_pool);
    });
}
ic_cdk::export_candid!();
