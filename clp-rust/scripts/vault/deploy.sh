set -e

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

#[deploy][vault][start]
did_file="$(pwd)/src/vault/vault.did"
dfx canister  $ic_flag create vault
candid-extractor "target/wasm32-unknown-unknown/release/vault.wasm" > $did_file
dfx generate vault
staking_pool="principal \"$(dfx canister  $ic_flag id staking_pool)\""
dfx deploy  $ic_flag vault -m reinstall --with-cycles 10000000000  -y --argument "(record { timer_interval_secs = 28800 : nat64; staking_pool = $staking_pool })"
echo "step addUnderlying =>"
dfx canister  $ic_flag call vault addUnderlying  "(principal \"$(dfx canister  $ic_flag id ckbtc)\",true,${ckbtc_fee},${ckbtc_cost},\"ckBTC\",${liquidate_rate})"
dfx canister  $ic_flag call vault addUnderlying "(principal \"$(dfx canister  $ic_flag id cketh)\",true,${cketh_fee},${cketh_cost},\"ckETH\",${liquidate_rate})"
echo "step setUnderlying =>"
#[deploy][vault][end]
dfx canister  $ic_flag call vault setMockPrice "(principal \"$(dfx canister  $ic_flag id ckbtc)\",6_934_535_000_000)"
dfx canister  $ic_flag call vault setMockPrice "(principal \"$(dfx canister  $ic_flag id cketh)\",369_094_000_000)"