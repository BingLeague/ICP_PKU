set -e
export clpt=$(dfx canister id clpt)
export meme=$(dfx identity get-principal)
period="30"
daily_reward=100_000_000_000_000
min_amount=1_000_000

user="principal \"$meme\""
account="record{ owner = $user; subaccount = null }"

auto_investment=true

balance=$(dfx canister call clpt icrc1_balance_of "($account)" | sed 's/[()]//g')

dfx canister call clpt icrc2_approve "(
        record {
            amount=$balance;
            created_at_time=null;
            expected_allowance=null;
            expires_at=null;
            fee=null;
            from_subaccount=null;
            memo=null;
            spender=record {
                owner = principal \"$(dfx canister id staking_pool)\";
                subaccount = null;
            }
        }
)"

balance=$(dfx canister call clpt icrc1_balance_of "($account)" | sed 's/[()]//g')

dfx canister call staking_pool stake "($period, $balance, $auto_investment)"



