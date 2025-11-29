#!/usr/bin/env bash

# set -x
set -euo pipefail

# Throw an error message and exit.
function raise() {
    echo "$1"
    exit 1;
}

# Check for positional arguments.
if [[ $# -eq 0 ]]; then
    raise 'Usage: run.sh /path/to/bitcoind/datadir'
fi

# Path to bitcoind data directory.
path=$1

# Terminate existing processes.
killall bitcoind > /dev/null 2>&1 || true

# Set environment variables used for integration testing.
export RPC_URL="127.0.0.1:18443"
export RPC_COOKIE="${path}/regtest/.cookie"

# Clear data directory.
rm -rf "${path}/regtest"

# Start bitcoind in Regtest.
bitcoind -regtest

sleep 1

# Create wallet.
echo 'Creating test wallet'
bitcoin-cli -regtest createwallet test > /dev/null

# Mine blocks.
echo 'Generating blocks...'
bitcoin-cli -regtest -generate 101 > /dev/null

# Run integration tests.
cargo test --test test_rpc_client -- --test-threads=1
# cargo test --test test_rpc_client -- --test-threads=1 --show-output
# cargo test --test test_rpc_client -- test_get_blockchain_info --test-threads=1

# Stop daemon.
bitcoin-cli -regtest stop
