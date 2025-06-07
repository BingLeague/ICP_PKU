#!/usr/bin/env bash
set -e

dfx canister call reserve_pool userBalance "(principal \"$(dfx identity get-principal)\")"

dfx canister call reserve_pool userUnderlyingBalances "(principal \"$(dfx identity get-principal)\")"

echo "default user cusd balance => "

dfx canister call cusd icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity --identity default get-principal)\";
                    subaccount = null;
                })"

echo "test_user user cusd balance => "

dfx canister call cusd icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity --identity test_user get-principal)\";
                    subaccount = null;
                })"