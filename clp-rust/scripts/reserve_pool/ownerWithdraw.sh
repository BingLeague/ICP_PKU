#!/usr/bin/env bash
set -e
ckBTC=$(dfx canister id ckbtc)
ckETH=$(dfx canister id cketh)
dfx canister call reserve_pool withdrawLiquidateUnderlying "(principal \"${ckBTC}\",19_990)"