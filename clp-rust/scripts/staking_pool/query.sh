set -e
export clpt=$(dfx canister id clpt)
export meme=$(dfx identity get-principal)

user="principal \"$meme\""
account="record{ owner = $user; subaccount = null }"
dfx canister call clpt icrc1_balance_of "($account)" | sed 's/[()]//g'


dfx canister call staking_pool allClptBalances

dfx canister call staking_pool balance "($user)"

dfx canister call staking_pool periodConfig