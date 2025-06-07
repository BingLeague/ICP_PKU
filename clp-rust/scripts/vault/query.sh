#!/usr/bin/env bash
set -e

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

ckBTC=$(dfx canister id ckbtc $ic_flag)
ckETH=$(dfx canister id cketh $ic_flag)

echo "user vault balances =>"
dfx canister call vault balances "(principal \"$(dfx identity get-principal)\")" $ic_flag

echo "user vault maxBorrow =>"
dfx canister call vault maxBorrow "(principal \"$(dfx identity get-principal)\",principal \"${ckBTC}\")" $ic_flag

echo "vault in ckbtc balance=>"
dfx canister call ckbtc icrc1_balance_of "(record {
                    owner = principal \"$(dfx canister id vault $ic_flag)\";
                    subaccount = null;
                })" $ic_flag
echo "user ckbtc balance => "
dfx canister call ckbtc icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity get-principal $ic_flag)\";
                    subaccount = null;
                })" $ic_flag

echo "user cusd balance => "
dfx canister call cusd icrc1_balance_of "(record {
                    owner = principal \"$(dfx identity get-principal)\";
                    subaccount = null;
                })" $ic_flag
echo "user cusd fee balance => "
dfx canister call cusd fee_balance $ic_flag

echo "vault vault fee balance  => "
dfx canister call vault feeBalance $ic_flag

