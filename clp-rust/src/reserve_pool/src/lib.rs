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
use std::iter::FromIterator;
use std::result::Result as StdResult;

mod deposit;
mod liquidate;
mod memory;
mod operations;
mod query;
mod set;
mod token;
mod types;
mod utils;
mod withdraw;

use types::Error;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct AssetInfo {
    asset_add: u128,
    cusd_sub: u128,
    wait_withdrawal: u128,
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
struct UserInfo {
    balance: u128,
    underlies: HashMap<Principal, AssetInfo>,
    total_sub: u128,
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

#[derive(Serialize, Deserialize)]
struct State {
    custodians: HashSet<Principal>,
    cusd: Option<Principal>,
    vault: Option<Principal>,
    liquidate_underlying: HashMap<Principal, u128>,
    balance: u128,
    total_sub: u128,
    need_fee: u128,
    staking_pool: Option<Principal>,
    fee: u128,
    #[serde(skip, default = "init_stable_data")]
    users: StableBTreeMap<Principal, UserInfo, Memory>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            custodians: HashSet::new(),
            cusd: None,
            vault: None,
            users: init_stable_data(),
            liquidate_underlying: HashMap::new(),
            balance: 0,
            total_sub: 0,
            need_fee: 0,
            staking_pool: None,
            fee: 0,
        }
    }
}

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

#[derive(CandidType, Deserialize)]
struct InitArgs {
    cusd: Principal,
    vault: Principal,
    staking_pool: Principal,
}

#[init]
fn init(args: InitArgs) {
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.custodians = HashSet::from_iter([api::caller()]);
        state.cusd = Some(args.cusd);
        state.vault = Some(args.vault);
        state.staking_pool = Some(args.staking_pool);
    });
}

fn init_stable_data() -> StableBTreeMap<Principal, UserInfo, Memory> {
    StableBTreeMap::init(crate::memory::get_stable_btree_memory())
}

pub type Result<T = u128, E = Error> = StdResult<T, E>;

ic_cdk::export_candid!();
