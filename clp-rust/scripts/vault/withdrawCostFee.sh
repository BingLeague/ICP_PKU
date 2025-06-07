fee=$(dfx canister call vault feeBalance);
echo "withdrawFee ${fee}=>"
dfx canister call vault withdrawFee "${fee}"