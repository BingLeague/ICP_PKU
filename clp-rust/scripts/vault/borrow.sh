#!/usr/bin/env bash
set -e

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

ckBTC=$(dfx canister id ckbtc $ic_flag)
ckETH=$(dfx canister id cketh $ic_flag)
echo "borrow=>"

dfx canister call vault borrow "(principal \"${ckBTC}\",1_306_995_612_833)" $ic_flag