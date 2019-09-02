#!/usr/bin/env bash

set -e

echo "*** Build chain"

# ./scripts/init.sh
cargo build --release

echo "*** Clean chain data"

./target/release/skystrate-node purge-chain --dev -y

echo "*** Start a new chain"

./target/release/skystrate-node --dev

