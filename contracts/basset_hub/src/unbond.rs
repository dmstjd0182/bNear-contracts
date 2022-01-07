use near_sdk::json_types::{U128};
use near_sdk::{near_bindgen, ext_contract, AccountId};

use crate::*;

#[ext_contract(ext_fungible_token)]
pub trait FungibleTokenContract {
    fn near_withdraw(&mut self, amount: U128) -> Promise;
}

#[near_bindgen]
impl Contract{
    fn execute_unbond(&self, amount: U128, sender_id: AccountId) {
        let epoch_period = self.parameters.epoch_period;
        let threshold = self.parameters.er_threshold;
        let recovery_fee = self.parameters.peg_recovery_fee;

        let mut current_batch = self.current_batch;

        // self.slashing();

        let mut state = self.state;

        let mut total_supply: u128 = self.query_total_issued().into();

        let amount_with_fee: U128;
        if state.exchange_rate < threshold {
            let max_peg_fee: f64 = (amount as f64) * recovery_fee;
            let required_peg_fee: u128 = (total_supply + current_batch.requested_with_fee)
                .checked_sub(state.total_bond_amount).unwrap();
            let peg_fee = std::cmp::min(max_peg_fee, required_peg_fee);
            amount_with_fee = amount.checked_sub(peg_fee);
        } else {
            amount_with_fee = amount;
        }
        current_batch.requested_with_fee += amount_with_fee;

        // store_unbond_wait_list(current_batch.id, sender_id.clone(), amount_with_fee,)?;

        total_supply = (total_supply.checked_sub(amount))
            .expect("the required can not be more than the total supply");

        // Update exchange rate
        state.update_exchange_rate(total_supply, current_batch.requested_with_fee);

        let current_time = env::block_timestamp();
        let passed_time = current_time - state.last_unbonded_time;

        //TODO undelegate

        ext_fungible_token::near_withdraw(
            amount,
            self.config.token_contract.unwrap(),
            0,
            5_000_000_000_000,
        );
    }
}