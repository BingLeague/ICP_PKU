set -e
# rm -f .env

echo "===========SETUP========="
dfx identity new minter --storage-mode plaintext || true
export MINTER_ACCOUNT_ID=$(dfx --identity minter ledger account-id)
export MINTER_PRINCIPAL=$(dfx --identity minter identity get-principal)
export DEFAULT_ACCOUNT_ID=$(dfx --identity default ledger account-id)
export DEFAULT_PRINCIPAL=$(dfx --identity default identity get-principal)
export cketh_cost=2_000_000
export cketh_fee=2_000_000_000_000
export ckbtc_cost=2_000_000
export ckbtc_fee=10
export liquidate_rate=70

# deploy cketh
dfx deploy cketh --ic  --with-cycles 800_000_000_000 --argument "( record {
        token_name = \"local ckETH\";
        token_symbol = \"local ckETH\";
        decimals = 18;
        transfer_fee = 2_000_000_000_000;
        initial_mints = vec {
            record {
                account=record {
                    owner = principal \"$DEFAULT_PRINCIPAL\";
                    subaccount = null;
                };
                amount=100_000_000_000_000_000_000 : nat;
            }
        };                    
        minting_account = record {
            owner = principal \"$MINTER_PRINCIPAL\";
            subaccount = null;
        };
    })"
dfx canister --ic call  cketh icrc1_balance_of "(record {
                    owner = principal \"$DEFAULT_PRINCIPAL\";
                    subaccount = null;
                })"
# deploy ckbtc
dfx deploy --ic ckbtc --with-cycles 800_000_000_000 --argument "( record {
        token_name = \"local ckBTC\";
        token_symbol = \"local ckBTC\";
        decimals = 8;
        transfer_fee = 10;
        initial_mints = vec {
            record {
                account=record {
                    owner = principal \"$DEFAULT_PRINCIPAL\";
                    subaccount = null;
                };
                amount=100_000_000_000_000_000_000 : nat;
            }
        };                   
        minting_account = record {
            owner = principal \"$MINTER_PRINCIPAL\";
            subaccount = null;
        };
    })"
  dfx canister --ic call ckbtc icrc1_balance_of "(record {
                    owner = principal \"$DEFAULT_PRINCIPAL\";
                    subaccount = null;
                })"

dfx generate ckbtc

did_file="$(pwd)/src/vault/vault.did"
dfx canister --ic create vault
dfx build vault
candid-extractor "target/wasm32-unknown-unknown/release/vault.wasm" > $did_file
dfx generate vault
dfx deploy vault --ic  --with-cycles 800_000_000_000 --argument "(record { timer_interval_secs = 1_000 : nat64; liquidate_rate = 80 : nat8 })"
echo "step addUnderlying =>"
dfx canister --ic call vault addUnderlying  "(principal \"$(dfx canister --ic id ckbtc)\",true,${ckbtc_fee},${ckbtc_cost},\"ckBTC\",${liquidate_rate})"
dfx canister --ic call vault addUnderlying "(principal \"$(dfx canister --ic id cketh)\",true,${cketh_fee},${cketh_cost},\"ckETH\",${liquidate_rate})"
echo "step setUnderlyingPlatform =>"
dfx canister --ic call vault setUnderlyingPlatform "(principal \"$(dfx canister --ic id cketh)\",variant{Coinbase},\"ETH\",100)"
dfx canister --ic call vault setUnderlyingPlatform "(principal \"$(dfx canister --ic id cketh)\",variant{Coingecko},\"chain-key-ethereum\",100)"
dfx canister --ic call vault setUnderlyingPlatform "(principal \"$(dfx canister --ic id ckbtc)\",variant{Coinbase},\"BTC\",100)"
dfx canister --ic call vault setUnderlyingPlatform "(principal \"$(dfx canister --ic id ckbtc)\",variant{Coingecko},\"chain-key-bitcoin\",100)"

dfx canister --ic call vault setMockPrice "(principal \"$(dfx canister --ic id ckbtc)\",variant{Coingecko},6_934_535_000_000)"
dfx canister --ic call vault setMockPrice "(principal \"$(dfx canister --ic id cketh)\",variant{Coingecko},369_094_000_000)"

# deploy cusd
dfx deploy --ic --with-cycles 800_000_000_000 cusd --argument "( record {
        token_name = \"locale cusd\";
        token_symbol = \"locale cusd\";
        decimals = 8;
        transfer_fee = 10000;
        initial_mints = vec {};
        minting_account = record {
            owner = principal \"$(dfx canister --ic id vault)\";
            subaccount = null;
        };
    })"

dfx canister --ic call vault setCUSD  "(principal \"$(dfx canister --ic id cusd)\",10000)"

did_file="$(pwd)/src/reserve_pool/reserve_pool.did"
dfx canister --ic create reserve_pool
dfx build reserve_pool
candid-extractor "target/wasm32-unknown-unknown/release/reserve_pool.wasm" > $did_file

dfx deploy --ic --with-cycles 800_000_000_000 reserve_pool --argument "(record { cusd=principal \"$(dfx canister --ic id cusd)\";vault=principal \"$(dfx canister --ic id vault)\"})"

dfx canister --ic call vault setReservePool "(principal \"$(dfx canister --ic id reserve_pool)\")"
