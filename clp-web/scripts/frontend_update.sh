#!/usr/bin/env bash
set -e
pushd frontend
NODE_ENV=production npm run build
popd
dfx build frontend --ic
dfx canister install frontend --mode=reinstall --ic

echo "===== VISIT DEFI FRONTEND ====="
echo "http://localhost:4943?canisterId=$(dfx canister id frontend --ic)"
echo "===== VISIT DEFI FRONTEND ====="
