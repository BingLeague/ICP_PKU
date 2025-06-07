set -e
bash vault/deposit.sh
bash vault/borrow.sh
bash vault/updatePrice.sh
bash reserve_pool/ownerWithdraw.sh


dfx identity new test_user --storage-mode plaintext || true

bash scripts/vault/deposit.sh
bash scripts/vault/borrow.sh
bash scripts/reserve_pool/deposit.sh
bash scripts/vault/updatePrice.sh
bash scripts/reserve_pool/withdrawUnderlying.sh