#!/bin/bash

set +e


ic_flag=""
if [[ "$1" == "--ic" ]]; then
    ic_flag="--ic"
fi

# 定义要执行的命令的函数
execute_command() {
    local identity_name=$1
    # 创建新的身份，如果失败则退出
    if dfx identity new "$identity_name" --storage-mode plaintext -qqqq; then
        # 如果创建身份成功，执行charge脚本
        echo "create $identity_name"
    else
        echo "Error: skip create user $identity_name"
        return 1
    fi
    bash scripts/ckbtc/charge.sh "$identity_name" $ic_flag
    bash scripts/cketh/charge.sh "$identity_name" $ic_flag
    dfx identity use $identity_name
    bash scripts/vault/deposit.sh $ic_flag
    bash scripts/vault/borrow.sh $ic_flag
    bash scripts/reserve_pool/deposit.sh $ic_flag
}

# 初始化计数器
counter=2

# 循环200次
while [ $counter -le 4200 ]; do
    name="test_$counter"
    execute_command "$name"
    if [ $? -ne 0 ]; then
        # 如果命令执行失败，则跳过当前迭代
        ((counter++))
        continue
    fi
    ((counter++))
done

