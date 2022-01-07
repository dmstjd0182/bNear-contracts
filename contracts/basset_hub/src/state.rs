use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};

use near_decimal::d128;


#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct CurrentBatch {
    pub id: u64,
    pub requested_with_fee: U128,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct State {
    pub exchange_rate: d128,
    pub total_bond_amount: U128,
    pub last_index_modification: u64,
    pub prev_hub_balance: U128,
    pub actual_unbonded_amount: U128,
    pub last_unbonded_time: u64,
    pub last_processed_batch: u64,
}

impl State {
    pub fn update_exchange_rate(&mut self, total_issued: U128, requested_with_fee: U128) {
        let actual_supply: U128 = (total_issued.0 + requested_with_fee.0).into();
        if self.total_bond_amount.0 == 0 || actual_supply.0 == 0 {
            self.exchange_rate = d128!(1);
        } else {
            self.exchange_rate = d128!(self.total_bond_amount.0) / d128!(actual_supply.0);
        }
    }
}

