#!/bin/bash
near deploy \
    --accountId staking.bnear.synchro.testnet \
    --wasmFile res/staking_pool.wasm \
    --initFunction new \
    --initArgs '{"owner_id": "synchro.testnet", "stake_public_key": "ARZ4wNgwZ85rCAnRKESJGNLdM54acSyjQUucKewN9tCM", "reward_fee_fraction": {"numerator": 10, "denominator": 100}, "token_contract": "bnear.synchro.testnet"}'