use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{Balance};

use near_decimal::d128;


#[derive(BorshDeserialize, BorshSerialize)]
pub struct CurrentBatch {
    pub id: u64,
    pub requested_with_fee: Balance,
}

#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct State {
    pub exchange_rate: d128,
    pub total_bond_amount: Balance,
    pub last_index_modification: u64,
    pub prev_hub_balance: Balance,
    pub actual_unbonded_amount: Balance,
    pub last_unbonded_time: u64,
    pub last_processed_batch: u64,
}

impl State {
    pub fn update_exchange_rate(&mut self, total_issued: Balance, requested_with_fee: Balance) {
        let actual_supply: Balance = total_issued + requested_with_fee;
        if self.total_bond_amount == 0 || actual_supply == 0 {
            self.exchange_rate = d128!(1);
        } else {
            self.exchange_rate = d128!(self.total_bond_amount) / d128!(actual_supply);
        }
    }
}

