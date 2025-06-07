#!/usr/bin/env bash
set -e
echo "user cusd balance => "
dfx canister call cusd icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity get-principal)\";
                    subaccount = null;
                })"
echo "user cusd fee balance => "
dfx canister call cusd fee_balance
