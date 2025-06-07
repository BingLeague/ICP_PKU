#!/bin/bash

# 检查是否提供了参数
if [ -z "$1" ]; then
    echo "Error: No user provided. Please supply a user principal as the first argument."
    exit 1
fi

user=$1

# 检查用户参数是否以'test'开头
if [[ "$user" == test* ]]; then
    # 使用dfx identity get-principal获取principal
    principal=$(dfx identity --identity=$user  get-principal) # 假设使用'default'作为身份名称，或者可以传递$user（去掉'test'前缀）作为身份名称
    if [ -z "$principal" ]; then
        echo "Error: Failed to get principal from dfx identity."
        exit 1
    fi
    user="$principal" # 将获取到的principal赋值给user变量
fi

# 检查是否提供了 --ic 参数
ic_flag=""
if [[ "$2" == "--ic" ]]; then
    ic_flag="--ic"
fi

# 执行 dfx 命令
dfx canister call ckbtc $ic_flag --identity minter icrc1_transfer "(
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
