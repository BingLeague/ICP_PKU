#!/usr/bin/env bash
set -e
# echo "approve"
# dfx canister call cusd icrc1_transfer "(
#     record {
#         amount= 968_824_900;
#         created_at_time= null;
#         fee=null;
#         from_subaccount= null;
#         memo=null;
#         to = record {
#             owner= principal \"$(dfx identity --identity test_user get-principal)\";
#             subaccount = null;
#         }
#     }
# )"

# dfx identity use test_user

ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

dfx canister call cusd icrc2_approve "(
        record {
            amount=1_306_993_612_733:nat;
            created_at_time=null;
            expected_allowance=null;
            expires_at=null;
            fee=null;
            from_subaccount=null;
            memo=null;
            spender=record {
                owner = principal \"$(dfx canister id reserve_pool $ic_flag)\";
                subaccount = null;
            }
        }
)" $ic_flag
echo "deposit"
dfx canister call reserve_pool deposit "(1_306_993_602_733)" $ic_flag