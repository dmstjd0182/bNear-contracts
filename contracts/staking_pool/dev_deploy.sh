#!/bin/bash
near dev-deploy \
    --wasmFile res/staking_pool.wasm \
    --initFunction new \
    --initArgs '{"owner_id": "blockwave.testnet", "stake_public_key": "AhHcd7La6X1CvdpVpFcC5SuuvNSUgNBM6LEz16Z8jFxn", "reward_fee_fraction": {"numerator": 10, "denominator": 100}, "token_contract": "dev-1642667871246-92638421223336"}' \
    --initialBalance 100_000_000_000_000_000_000_000_000_000