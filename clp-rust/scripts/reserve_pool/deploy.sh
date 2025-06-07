set -e 
ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi
did_file="$(pwd)/src/reserve_pool/reserve_pool.did"
dfx canister $ic_flag create reserve_pool
dfx build reserve_pool
candid-extractor "target/wasm32-unknown-unknown/release/reserve_pool.wasm" > $did_file
dfx deploy $ic_flag reserve_pool -m reinstall -y --argument "(record { cusd=principal \"$(dfx canister $ic_flag id cusd)\";vault=principal \"$(dfx canister $ic_flag id vault)\";staking_pool=principal \"$(dfx canister $ic_flag id staking_pool)\";})"