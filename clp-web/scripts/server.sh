#!/usr/bin/env bash
set -e
cd frontend
(npm run dev > ../server.log 2>&1) &
# (NODE_ENV=production npm run dev > server.log 2>&1) &
pid=$!
echo "PID of npm run dev is: $pid"