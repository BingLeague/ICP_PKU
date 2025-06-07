#!/usr/bin/env bash
set -e

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

echo $ic_flag

ckBTC=$(dfx canister id ckbtc $ic_flag)
ckETH=$(dfx canister id cketh $ic_flag)
echo "approve=>"
dfx canister call ckbtc icrc2_approve "(
        record {
            amount=100_000_000:nat;
            created_at_time=null;
            expected_allowance=null;
            expires_at=null;
            fee=null;
            from_subaccount=null;
            memo=null;
            spender=record {
                owner = principal \"$(dfx canister id vault $ic_flag)\";
                subaccount = null;
            }
        }
)" $ic_flag

# dfx canister call ckbtc icrc2_allowance "(
#         record {
#             account=record {
#                 owner = principal \"$(dfx identity get-principal $ic_flag)\";
#                 subaccount = null;
#             };
#             spender=record {
#                 owner = principal \"$(dfx canister id vault $ic_flag)\";
#                 subaccount = null;
#             }
#         }
# )" $ic_flag

echo "deposit=>"
dfx canister call vault deposit "(principal \"${ckBTC}\",90_000_000)" $ic_flag