#!/bin/env bash --ic 
set -e

echo "===========SETUP========="
dfx identity use default
export MINTER_ACCOUNT_ID=$(dfx --identity minter ledger account-id)
export MINTER_PRINCIPAL=$(dfx --identity minter identity get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)
export DEFAULT_PRINCIPAL=$(dfx --identity default identity get-principal)
export cketh_cost=2_000_000
export cketh_fee=2_000_000_000_000
export ckbtc_cost=2_000_000
export ckbtc_fee=10
export liquidate_rate=70

echo "-----staking_pool"
# bash scripts/staking_pool/deploy.sh  --ic
# echo "-----clpt"
# bash  scripts/clpt/deploy.sh --ic
# echo "-----cketh"
# bash  scripts/cketh/deploy.sh  --ic
# echo "-----ckbtc"
# bash  scripts/ckbtc/deploy.sh  --ic
# echo "-----vault"
# bash  scripts/vault/deploy.sh  --ic
# echo "-----cusd"
# bash  scripts/cusd/deploy.sh --ic
# echo "-----reserve_pool"
# bash  scripts/reserve_pool/deploy.sh  --ic
{
    vault="principal \"$(dfx canister --ic  id vault )\""
    clpt="principal \"$(dfx canister --ic  id clpt )\""
    reserve_pool="principal \"$(dfx canister --ic  id reserve_pool )\""
    dfx canister --ic call staking_pool paramInit "($vault,$clpt,$reserve_pool)" 
}
{
    cusd="principal \"$(dfx canister --ic  id cusd )\""
    dfx canister --ic  call vault setCUSD  "($cusd,10000)" 
}

dfx canister --ic  call reserve_pool setFee "(10_000)" 
{
    reserve_pool="principal \"$(dfx canister --ic  id reserve_pool)\""
    dfx canister --ic  call vault setReservePool "($reserve_pool)" 
}
{
    #ckbtc deposit
    price="100_000_000"
    blob='"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"'
    account="record{ owner =principal \"$(dfx canister --ic  id ckbtc)\"; subaccount = opt blob $blob; }"
    amount="200_000_000"
    dfx canister --ic  call staking_pool setAssetConfig "($account,$amount,$price)" 

    #ckbtc borrow
    blob='"\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"'
    account="record{ owner =principal \"$(dfx canister --ic  id ckbtc)\"; subaccount = opt blob $blob; }"
    amount="200_000_000"
    dfx canister --ic  call staking_pool setAssetConfig "($account,$amount,$price)" 

    #cketh deposit
    price="100_000_000"
    blob='"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"'
    account="record{ owner =principal \"$(dfx canister --ic  id cketh)\"; subaccount = opt blob $blob; }"
    amount="200_000_000"
    dfx canister --ic  call staking_pool setAssetConfig "($account,$amount,$price)" 

    #cketh borrow
    blob='"\02\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00"'
    account="record{ owner =principal \"$(dfx canister --ic  id cketh)\"; subaccount = opt blob $blob; }"
    amount="200_000_000"
    dfx canister --ic  call staking_pool setAssetConfig "($account,$amount,$price)" 

    #reserve pool
    account="record{ owner =principal \"$(dfx canister --ic  id cusd)\"; subaccount = null; }"
    amount="200_000_000"
    dfx canister --ic  call staking_pool setAssetConfig "($account,$amount,$price)" 
}

dfx canister --ic  call vault collateralInspection 