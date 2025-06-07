# 检查端口是否开放
PORT=4943
TIMEOUT=1  # 设置超时时间，单位是秒

# 使用curl检查端口是否开放
if ! timeout $TIMEOUT curl -s -o /dev/null -w "%{http_code}" http://localhost:$PORT/ &> /dev/null; then
    # 如果curl命令超时或返回非200状态码，则认为端口未开启
    echo "端口 $PORT 未开启"
else
    # 如果curl命令成功执行并返回200状态码，则认为端口已开启
    echo "端口 $PORT 已开启，无需启动服务。"
fi