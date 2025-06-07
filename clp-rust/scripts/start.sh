#!/bin/env bash
set -e
# rm -f .env
cd ~
# 检查端口是否开放
PORT=8000
TIMEOUT=1  # 设置超时时间，单位是秒

# 使用curl检查端口是否开放
if ! timeout $TIMEOUT curl -s -o /dev/null -w "%{http_code}" http://localhost:$PORT/ &> /dev/null; then
    # 如果curl命令超时或返回非200状态码，则认为端口未开启
    echo "端口 $PORT 未开启，正在启动服务..."
    # 这里假设dfx start是你的服务启动命令，你需要替换成你自己的服务启动命令
    dfx start --host 0.0.0.0:$PORT  --background --clean > /canister/clp/logs/dfx.log 2>&1
    echo "服务启动完成，日志已记录到 logs/dfx.log"
else
    # 如果curl命令成功执行并返回200状态码，则认为端口已开启
    echo "端口 $PORT 已开启，无需启动服务。"
fi