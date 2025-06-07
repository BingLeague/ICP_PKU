set -e
# deploy internet_identity
II_FETCH_ROOT_KEY=1 dfx deploy internet_identity -m reinstall -y --no-wallet --argument '(null)'