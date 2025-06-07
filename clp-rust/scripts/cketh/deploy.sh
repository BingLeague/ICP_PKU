set -e
ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

# deploy cketh
dfx deploy $ic_flag cketh -m reinstall -y --argument "( record {
        token_name = \"ckETH\";
        token_symbol = \"ckETH\";
        decimals = 18;
        transfer_fee = 2_000_000_000_000;
        initial_mints = vec {
            record {
                account=record {
                    owner = principal \"$DEFAULT_PRINCIPAL\";
                    subaccount = null;
                };
                amount=100_000_000_000_000_000_000 : nat;
            }
        };                    
        minting_account = record {
            owner = principal \"$MINTER_PRINCIPAL\";
            subaccount = null;
        };
    })"