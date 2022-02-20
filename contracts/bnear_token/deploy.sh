#!/bin/bash
./build.sh
near deploy \
    --accountId bnear.synchro.testnet \
    --wasmFile res/bnear_token.wasm \
    --initFunction new \
    --initArgs '{"staking_pool": "staking.bnear.synchro.testnet"}'