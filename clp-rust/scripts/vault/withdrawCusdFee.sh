fee=$(dfx canister call cusd fee_balance);
echo "withdrawCusdFee ${fee}=>"
dfx canister call vault withdrawCusdFee "${fee}"