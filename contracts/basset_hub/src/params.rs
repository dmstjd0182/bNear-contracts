use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_decimal::d128;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Parameters {
    pub epoch_period: u64,
    pub underlying_coin_denom: String,
    pub unbonding_period: u64,
    pub peg_recovery_fee: d128,
    pub er_threshold: d128,
    pub reward_denom: String,
}