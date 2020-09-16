#!/usr/bin/env bash

set -e

echo "*** Start Substrate node template ***"

cd $(dirname ${BASH_SOURCE[0]})/..
cargo build --release
cp target/release/node-template scripts/substrate
cd scripts
docker build -t foo .
rm substrate