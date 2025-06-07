#!/usr/bin/env bash
set -e

ckBTC=$(dfx canister id ckbtc)
ckETH=$(dfx canister id cketh)
echo "approve=>"
dfx canister call cusd icrc2_approve "(
        record {
            amount=50_417_150:nat;
            created_at_time=null;
            expected_allowance=null;
            expires_at=null;
            fee=null;
            from_subaccount=null;
            memo=null;
            spender=record {
                owner = principal \"$(dfx canister id vault)\";
                subaccount = null;
            }
        }
)"
echo "repayment=>"
dfx canister call vault repayment "(principal \"${ckBTC}\",5_417_450)"