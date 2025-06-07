export MINTER_ACCOUNT_ID=$(dfx --identity minter ledger account-id)
export MINTER_PRINCIPAL=$(dfx --identity minter identity get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)
export DEFAULT_PRINCIPAL=$(dfx --identity default identity get-principal)
dfx build ckbtc
dfx canister install  ckbtc --mode upgrade --argument "( record {
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