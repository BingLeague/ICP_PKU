#!/usr/bin/env bash
set -e
# did_file="$(pwd)/src/reserve_pool/reserve_pool.did"
# candid-extractor "target/wasm32-unknown-unknown/release/reserve_pool.wasm" > $did_file
# dfx build reserve_pool --ic
# dfx canister install reserve_pool --mode upgrade --argument "(record { cusd=principal \"$(dfx canister id cusd)\";vault=principal \"$(dfx canister id vault)\"})" --ic


did_file="$(pwd)/src/reserve_pool/reserve_pool.did"
candid-extractor "target/wasm32-unknown-unknown/release/reserve_pool.wasm" > $did_file
dfx build reserve_pool
dfx canister install reserve_pool --mode upgrade --argument "(record { cusd=principal \"$(dfx canister id cusd)\";vault=principal \"$(dfx canister id vault)\";staking_pool=principal \"$(dfx canister id staking_pool)\";})"