use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc, PanicOnDefault, BorshStorageKey,
    AccountId, Balance};
use near_sdk::json_types::{U128};
use near_sdk::collections::{LookupMap, Vector};
use near_decimal::d128;

use crate::config::Config;
use crate::state::{State, CurrentBatch};
use crate::params::Parameters;
use crate::unbond::UnbondHistory;

mod config;
mod params;
mod state;
mod token_receiver;
mod unbond;
mod utils;
mod views;

setup_alloc!();

#[derive(BorshStorageKey, BorshSerialize)]
pub enum StorageKeys {
    UnbondWaitList,
    Account { account_hash: Vec<u8> },
    UnbondHistoryKey,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    config: Config,
    parameters: Parameters,
    current_batch: CurrentBatch,
    state: State,
    unbond_wait_list: LookupMap<AccountId, LookupMap<u64, U128>>,
    unbond_history: Vector<UnbondHistory>,
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
            id: 0,
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
            unbond_wait_list: LookupMap::new(StorageKeys::UnbondWaitList),
            unbond_history: Vector::new(StorageKeys::UnbondHistoryKey),
        }
    }

    // Check slashing, update state, and calculate the new exchange rate.
    fn slashing(&mut self) {

        // Check the amount that contract thinks is bonded
        let state_total_bonded: U128 = self.state.total_bond_amount;

        // Check the actual bonded amount
        let delegations: Balance = env::validator_stake(&env::current_account_id());
        let actual_total_bonded = U128(delegations);

        // Need total issued for updating the exchange rate
        let total_issued: U128 = U128(0);
        self.get_total_supply(total_issued);
        let current_requested_fee = self.current_batch.requested_with_fee;
        
        // Slashing happened if the actual amount is less than stored amount
        if state_total_bonded.0 > actual_total_bonded.0 {
            self.state.total_bond_amount = actual_total_bonded;
            self.state.update_exchange_rate(total_issued, current_requested_fee);
        }
    }

    /// Store undelegation wait list per each batch
    /// LookupMap<user's address, LookupMap<batch_id, requested_amount>>
    fn internal_store_unbond_wait_list(
        &mut self,
        batch_id: u64,
        sender_account_id: AccountId,
        amount: U128,
    ) {
        // Get nested LookupMap
        let mut sender_wait_list = self.unbond_wait_list.get(&sender_account_id).unwrap_or_else(|| {
            LookupMap::new(
                StorageKeys::Account{ account_hash: env::sha256(sender_account_id.as_bytes()) }
            )
        });
        // Add amount
        let new_value: u128 = sender_wait_list.get(&batch_id).unwrap_or(U128(0)).0 + amount.0;

        // Save maps
        sender_wait_list.insert(&batch_id, &U128(new_value));
        self.unbond_wait_list.insert(&sender_account_id, &sender_wait_list);
    }

    /// Store unbond history map
    /// Vector<batch_id, UnbondHistory>
    fn internal_store_unbond_history(&mut self, history: UnbondHistory) {
        self.unbond_history.push(&history);
    }
}