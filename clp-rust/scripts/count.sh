#!/bin/bash

# 文件路径
FILE="users.txt"

# 提取并去重 principal 值
unique_principals=$(grep -oP 'principal\s*"\K[^"]+' "$FILE" | sort -u)

# 计算唯一 principal 的数量
count=$(echo "$unique_principals" | wc -l)

# 输出结果
echo "总共有 $count 个唯一的 principal"