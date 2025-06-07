#!/usr/bin/env bash
set -e
dfx build vault
did_file="$(pwd)/src/vault/vault.did"
candid-extractor "target/wasm32-unknown-unknown/release/vault.wasm" > $did_file
staking_pool="principal \"$(dfx canister id staking_pool)\""
dfx canister install  vault --mode upgrade --argument "(record { timer_interval_secs = 60 : nat64; staking_pool = $staking_pool })"