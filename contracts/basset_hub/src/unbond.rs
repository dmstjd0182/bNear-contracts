use near_sdk::json_types::{U128};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, AccountId, BlockHeight, assert_self};
use near_decimal::d128;

use crate::*;
use crate::utils::{ext_fungible_token};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UnbondHistory {
    pub batch_id: u64,
    pub time: u64,
    pub amount: U128,
    pub applied_exchange_rate: d128,
    pub withdraw_rate: d128,
    pub released: bool,
}

#[near_bindgen]
impl Contract{
    /// This must be called by ft_on_transfer
    /// This will undelegate NEAR and burn basset token
    pub fn execute_unbond(&mut self, amount: U128, sender_id: AccountId) {
        assert_self();
        let epoch_period: u64 = self.parameters.epoch_period;
        let threshold: d128 = self.parameters.er_threshold;
        let recovery_fee: d128 = self.parameters.peg_recovery_fee;

        let mut current_batch = self.current_batch;

        // Check slashing, update state, and calculate the new exchange rate.
        self.slashing();

        let mut state = self.state;

        let mut total_supply: U128 = U128(0);
        self.get_total_supply(total_supply);

        // Collect all the requests within a epoch period
        // Apply peg recovery fee
        let amount_with_fee: U128;
        if state.exchange_rate < threshold {
            let max_peg_fee: d128 = d128!(amount.0) * recovery_fee;
            let required_peg_fee: u128 = (total_supply.0 + current_batch.requested_with_fee.0)
                .checked_sub(state.total_bond_amount.0).unwrap();
            let peg_fee: u128 = std::cmp::min(max_peg_fee.into(), required_peg_fee);
            amount_with_fee = U128((amount.0).checked_sub(peg_fee).unwrap());
        } else {
            amount_with_fee = amount;
        }
        current_batch.requested_with_fee = U128(current_batch.requested_with_fee.0 + amount_with_fee.0);

        self.internal_store_unbond_wait_list(current_batch.id, sender_id.clone(), amount_with_fee);

        total_supply = U128((total_supply.0.checked_sub(amount.0))
            .expect("the required amount can not be more than the total supply"));

        // Update exchange rate
        state.update_exchange_rate(total_supply, current_batch.requested_with_fee);

        let current_time = env::block_timestamp();
        let passed_time = current_time - state.last_unbonded_time;

        // If the epoch period is passed, the undelegate message would be sent.
        if passed_time > epoch_period {
            //Apply the current exchange rate.
            let undelegation_amount: d128 = d128!(current_batch.requested_with_fee.0) * state.exchange_rate;
            assert!(undelegation_amount > d128!(1), "Burn amount must be greater than 1 yoctoNear");

            let delegator: AccountId = env::current_account_id();

            let block_height: BlockHeight = env::block_index();
            
            //TODO undelegate

            state.total_bond_amount = U128(state.total_bond_amount.0.checked_sub(undelegation_amount.into()).unwrap());

            // Store history for withdraw unbonded
            let history = UnbondHistory {
                batch_id: current_batch.id,
                time: env::block_timestamp(),
                amount: current_batch.requested_with_fee,
                applied_exchange_rate: state.exchange_rate,
                withdraw_rate: state.exchange_rate,
                released: false,
            };
            self.internal_store_unbond_history(history);

            // batch info must be updated to new batch
            current_batch.id += 1;
            current_batch.requested_with_fee = U128(0);

            // state.last_unbonded_time must be updated to the current block time
            state.last_unbonded_time = env::block_timestamp();
        }

        // Burn bNear
        ext_fungible_token::burn(
            amount,
            &self.config.token_contract.as_ref().unwrap(),
            0,
            5_000_000_000_000,
        );
    }
}