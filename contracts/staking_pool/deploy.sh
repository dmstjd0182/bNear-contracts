#!/bin/bash
near deploy \
    --accountId staking.blockwave.testnet \
    --wasmFile res/staking_pool.wasm \
    --initFunction new \
    --initArgs '{"owner_id": "blockwave.testnet", "stake_public_key": "EycV9zs7ZUk1jVvzUyx13Rv6oEqyZqyTb9AQB74NdQPd", "reward_fee_fraction": {"numerator": 10, "denominator": 100}, "token_contract": "bnear.blockwave.testnet"}'