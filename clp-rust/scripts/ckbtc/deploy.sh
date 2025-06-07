set -e
ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi
# deploy ckbtc
dfx deploy $ic_flag ckbtc -m reinstall -y --argument "( record {
        token_name = \"ckBTC\";
        token_symbol = \"ckBTC\";
        decimals = 8;
        transfer_fee = 10;
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