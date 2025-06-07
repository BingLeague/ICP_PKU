set -e
did_file="$(pwd)/src/staking_pool/staking_pool.did"
dfx canister create staking_pool
dfx build staking_pool
dfx generate staking_pool
candid-extractor "target/wasm32-unknown-unknown/release/staking_pool.wasm" > $did_file
meme=$(dfx identity get-principal)

dfx deploy staking_pool -y  --argument "(record { timer_interval_secs = 60 : nat64;vault=principal \"$meme\" })"