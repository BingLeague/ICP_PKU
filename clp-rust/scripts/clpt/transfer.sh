user=""
# 执行 dfx 命令
dfx canister call clpt icrc1_transfer "(
        record {
            amount = 100_000_000;
            created_at_time = null;
            fee = null;
            from_subaccount = null;
            memo = null;
            to = record {
                owner = principal \"$user\";
                subaccount = null;
            };
        }
)"