#!/bin/bash
./build.sh
near deploy \
    --accountId bnear.blockwave.testnet \
    --wasmFile res/bnear_token.wasm \
    --initFunction new \
    --initArgs '{"staking_pool": "staking.blockwave.testnet"}'