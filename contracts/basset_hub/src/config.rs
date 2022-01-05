use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Config {
    pub creator: AccountId,
    pub reward_contract: Option<AccountId>,
    pub token_contract: Option<AccountId>,
    pub airdrop_registry_contract: Option<AccountId>,
}