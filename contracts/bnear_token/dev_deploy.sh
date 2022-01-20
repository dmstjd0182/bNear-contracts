#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/bnear_token.wasm \
    --initFunction new \
    --initArgs '{"staking_pool": "dev-1642667924375-10674190632790"}'