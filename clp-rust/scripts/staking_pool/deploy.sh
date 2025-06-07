set -e

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

did_file="$(pwd)/src/staking_pool/staking_pool.did"
dfx generate staking_pool
candid-extractor "target/wasm32-unknown-unknown/release/staking_pool.wasm" > $did_file
meme=$(dfx identity get-principal)

dfx deploy $ic_flag staking_pool -m reinstall -y --argument "(record { timer_interval_secs = 60 : nat64;vault=principal \"$meme\" })"

export meme=$(dfx identity get-principal)
daily_reward=100_000_000_000_000
min_amount=1_000_000

dfx canister  $ic_flag call staking_pool setClptPrice "(10_000_000_000)"

dfx canister  $ic_flag call staking_pool setPeriodConfig "(7,$daily_reward,$min_amount)"
dfx canister  $ic_flag call staking_pool setPeriodConfig "(30,$daily_reward,$min_amount)"

dfx canister  $ic_flag call staking_pool setPeriodConfig "(60,$daily_reward,$min_amount)"

dfx canister  $ic_flag call staking_pool setPeriodConfig "(90,$daily_reward,$min_amount)"

dfx canister  $ic_flag call staking_pool setPeriodConfig "(180,$daily_reward,$min_amount)"