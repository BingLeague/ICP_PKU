set -e
ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

dfx deploy $ic_flag clpt -m reinstall -y --argument "( record {
        token_name = \"clpt\";
        token_symbol = \"clpt\";
        decimals = 8;
        transfer_fee = 10_000;
        initial_mints = vec {};
        minting_account = record {
            owner = principal \"$(dfx canister id staking_pool $ic_flag)\";
            subaccount = null;
        };
    })"
