user=$(dfx identity get-principal)
dfx canister call clpt icrc1_balance_of "(record {
                    owner = principal \"$user\";
                    subaccount = null;
                })"
