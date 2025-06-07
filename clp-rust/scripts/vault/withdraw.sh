#!/usr/bin/env bash
set -e

ckBTC=$(dfx canister id ckbtc)
ckETH=$(dfx canister id cketh)

echo "user before balance=>"

dfx canister call ckbtc icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity get-principal)\";
                    subaccount = null;
                })"

echo "use withdraw=>"

dfx canister call vault withdraw "(principal \"${ckBTC}\",20_000)"

dfx canister call ckbtc icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity get-principal)\";
                    subaccount = null;
                })"