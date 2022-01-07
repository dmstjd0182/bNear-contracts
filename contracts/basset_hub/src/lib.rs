use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault, PromiseOrValue, ext_contract,
     AccountId, Balance};
use near_sdk::json_types::{U128};
use near_decimal::d128;

use crate::config::Config;
use crate::state::{State, CurrentBatch};
use crate::params::Parameters;

mod config;
mod params;
mod state;
mod token_receiver;
// mod unbond;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    config: Config,
    parameters: Parameters,
    current_batch: CurrentBatch,
    state: State,
}

#[near_bindgen]
impl Contract {
    #[init]
    #[payable]
    pub fn new(
        epoch_period: u64,
        underlying_coin_denom: String,
        unbonding_period: u64,
        peg_recovery_fee: d128,
        er_threshold: d128,
        reward_denom: String
    ) -> Self {
        assert!(!env::state_exists(), "The contract is already initialized");

        let payment: Balance = match env::attached_deposit() {
            value if value > 0 => value,
            _ => env::panic(b"No assets are provided to bond"),
        };

        // store config
        let config = Config {
            creator: env::signer_account_id(),
            reward_contract: None,
            token_contract: None,
            airdrop_registry_contract: None,
        };

        // instantiate parameters
        let parameters = Parameters {
            epoch_period,
            underlying_coin_denom,
            unbonding_period,
            peg_recovery_fee,
            er_threshold,
            reward_denom,
        };

        let current_batch = CurrentBatch {
            id: 1,
            requested_with_fee: U128(0),
        };

        // store state
        let state = State {
            exchange_rate: d128!(1),
            last_index_modification: env::block_timestamp(),
            last_unbonded_time: env::block_timestamp(),
            last_processed_batch: 0u64,
            total_bond_amount: payment.into(),
            prev_hub_balance: U128(0),
            actual_unbonded_amount: U128(0)
        };

        Self{
            config,
            parameters,
            current_batch,
            state,
        }
    }

    // Check slashing, update state, and calculate the new exchange rate.
    fn slashing(&mut self) {
        let coin_denom = self.parameters.underlying_coin_denom;

        // Check the amount that contract thinks is bonded
        let state_total_bonded: U128 = self.state.total_bond_amount;

        // Check the actual bonded amount
        let delegations: Balance = env::validator_stake(env::current_account_id().as_ref());
        let mut actual_total_bonded = U128(delegations);

        // Need total issued for updating the exchange rate
        let total_issued: U128 = self.get_total_supply();
        let current_requested_fee = self.current_batch.requested_with_fee;
        
        // Slashing happened if the actual amount is less than stored amount
        if state_total_bonded.0 > actual_total_bonded.0 {
            self.state.total_bond_amount = actual_total_bonded;
            state.update_exchange_rate(total_issued, current_requested_fee);
        }
    }
}