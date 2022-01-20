#!/bin/bash
./build.sh
near dev-deploy \
    --wasmFile res/staking_pool.wasm \
    --initFunction new \
    --initArgs '{"owner_id": "blockwave.testnet", "stake_public_key": "47eeCf1L9oHp6J3qn4vLaWPogzfrzxZgikW2mBegW8Nd", "reward_fee_fraction": {"numerator": 10, "denominator": 100}, "token_contract": "dev-1642678270546-91717094400488"}' \
    --initialBalance 100_000_000_000_000_000_000_000_000_000