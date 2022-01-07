use near_sdk::{near_bindgen};
use near_sdk::json_types::{U128};

use crate::*;
use crate::utils::{ext_fungible_token};

#[near_bindgen]
impl Contract {
    pub fn get_total_supply() -> U128 {
        ext_fungible_token::ft_total_supply(
            self.config.token_contract.unwrap(),
            0,
            5_000_000_000_000,
        )
    }
}