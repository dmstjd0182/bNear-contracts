use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{near_bindgen, env, PromiseOrValue};
use near_sdk::json_types::{U128, ValidAccountId};

use crate::*;

#[near_bindgen]
impl FungibleTokenReceiver for Contract{
    /// Callback on receiving tokens by this contract.
    #[allow(unreachable_code)]
    fn ft_on_transfer(
        &mut self,
        sender_id: ValidAccountId,
        amount: U128,
        _msg: String,
    ) -> PromiseOrValue<U128> {
        assert!(self.config.token_contract == Some(env::predecessor_account_id()),
            "the token contract must have been registered");

        self.execute_unbond(amount, sender_id.into());
        // we don't return funds back to sender.
        PromiseOrValue::Value(U128(0))
    }
}