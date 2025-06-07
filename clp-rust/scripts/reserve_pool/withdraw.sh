#!/usr/bin/env bash
set -e


dfx identity use test_user
dfx canister call reserve_pool withdraw "(968_704_900)"
dfx identity use default