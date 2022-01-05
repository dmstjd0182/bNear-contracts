use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault};

use crate::config::Config;
use crate::state::{State, CurrentBatch};
use crate::params::Parameters;

mod config;
mod state;
mod params;

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
    pub fn new(
        epoch_period: u64,
        underlying_coin_denom: String,
        unbonding_period: u64,
        peg_recovery_fee: f64,
        er_threshold: f64,
        reward_denom: String
    ) -> Self {
        let payment = env::attached_deposit();

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
            requested_with_fee: Default::default(),
        };

        // store state
        let state = State {
        exchange_rate: 1.0,
        last_index_modification: env::block_timestamp(),
        last_unbonded_time: env::block_timestamp(),
        last_processed_batch: 0u64,
        total_bond_amount: payment,
        ..Default::default()
        };

        Self{
            config,
            parameters,
            current_batch,
            state,
        }
    }
}
