#!/usr/bin/env bash
set -e
dfx canister call reserve_pool userUnderlyingBalances "(principal \"$(dfx identity get-principal)\")"
# dfx canister call reserve_pool withdrawUnderlying "(principal ${ckBTC},19_990)"