
dfx canister call vault setMockPrice "(principal \"$(dfx canister id ckbtc)\",6_934_534_000_000)"
dfx canister call vault setMockPrice "(principal \"$(dfx canister id cketh)\",369_093_000_000)"

dfx canister call vault collateralInspection