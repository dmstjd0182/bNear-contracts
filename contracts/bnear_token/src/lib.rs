use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};
use near_contract_standards::fungible_token::FungibleToken;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{ValidAccountId, U128};
use near_sdk::{env, near_bindgen, log, Balance, AccountId, PanicOnDefault, PromiseOrValue};

mod b_near;

near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
    pub staking_pool: AccountId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(staking_pool: ValidAccountId) -> Self {
        Self {
            ft: FungibleToken::new(b"f".to_vec()),
            staking_pool: staking_pool.into(),
        }
    }

    pub(crate) fn assert_staking_pool(&self) {
        assert_eq!(
            env::predecessor_account_id(),
            self.staking_pool,
            "Can only be called by the staking pool."
        );
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);
near_contract_standards::impl_fungible_token_storage!(Contract, ft);

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Bonded NEAR fungible token"),
            symbol: String::from("bNEAR"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 24,
        }
    }
}


#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;