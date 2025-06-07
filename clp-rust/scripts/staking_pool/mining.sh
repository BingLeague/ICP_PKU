set -e
is_add=true
meme=$(dfx identity get-principal)
user="principal \"$meme\""
account="record{ owner = $user; subaccount = null }"
amount="200_000_000"


dfx canister call staking_pool setAssetConfig "($account,$amount,100_000_000)"

dfx canister call staking_pool update_balance "($user,$account,$amount,$is_add)"

dfx canister call staking_pool allUserBalances

dfx canister call staking_pool assetConfig


dfx canister call staking_pool balance "($user)"

dfx canister call staking_pool withdraw