use near_sdk::{near_bindgen};
use near_sdk::json_types::{U128};

use crate::*;
use crate::utils::{ext_fungible_token, ext_self};

#[near_bindgen]
impl Contract {
    pub fn get_total_supply(&self, result: U128) {
        ext_fungible_token::ft_total_supply(
            &self.config.token_contract.as_ref().unwrap(),
            0,
            5_000_000_000_000,
        ).then(ext_self::callback_get_total_supply(
            result,
            &env::current_account_id(),
            0,
            5_000_000_000_000,
        ));
    }
}
