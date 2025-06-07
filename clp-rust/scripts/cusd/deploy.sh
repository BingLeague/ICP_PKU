set -e
ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi
# deploy cusd
dfx deploy $ic_flag cusd -m reinstall -y --argument "( record {
        token_name = \"cusd\";
        token_symbol = \"cusd\";
        decimals = 8;
        transfer_fee = 10_000;
        initial_mints = vec {};
        minting_account = record {
            owner = principal \"$(dfx canister $ic_flag id vault)\";
            subaccount = null;
        };
    })"