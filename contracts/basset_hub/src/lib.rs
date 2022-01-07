use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault, PromiseOrValue, ext_contract,
     AccountId, Balance};
use near_sdk::json_types::{U128};
use near_decimal::d128;

use crate::config::Config;
use crate::state::{State, CurrentBatch};
use crate::params::Parameters;

mod config;
mod state;
mod params;
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
            requested_with_fee: Default::default(),
        };

        // store state
        let state = State {
            exchange_rate: d128!(1),
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

    // pub fn ft_on_transfer(
    //     &mut self,
    //     sender_id: AccountId,
    //     amount: U128,
    //     msg: String,
    // ) -> PromiseOrValue<U128> {
    //     //only token contract can execute
    //     assert!(self.config.token_contract == env::predecessor_account_id());

    //     self.execute_unbond(amount, sender_id);
    // }

    // pub fn query_total_issued(&self) -> U128 {
    //     ext_fungible_token::ft_total_supply(
    //         self.config.token_contract.unwrap(),
    //         0,
    //         5_000_000_000_000,
    //     )
    // }

    pub fn set_number(&self, a: d128, b: d128) -> d128 {
        a * b
    }
}

#[test]
fn d128_test() {
    let instance: Contract = Contract{number_1: d128!(0.0), number_2: d128!(0.0)};
    assert_eq!(format!("{}", instance.set_number(d128!(0.7), d128!(0.184))), "0.1288");
}
