export MINTER_ACCOUNT_ID=$(dfx --identity minter ledger account-id)
export MINTER_PRINCIPAL=$(dfx --identity minter identity get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)
export DEFAULT_PRINCIPAL=$(dfx --identity default identity get-principal)
dfx build cusd
dfx canister install  cusd --mode upgrade --argument "( record {
        token_name = \"cusd\";
        token_symbol = \"cusd\";
        decimals = 8;
        transfer_fee = 10000;
        initial_mints = vec {};
        minting_account = record {
            owner = principal \"$(dfx canister id vault)\";
            subaccount = null;
        };
    })"