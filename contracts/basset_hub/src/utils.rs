use near_sdk::{near_bindgen, ext_contract, assert_self, PromiseResult};
use near_sdk::json_types::{U128, ValidAccountId};

use crate::*;

#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    fn ft_transfer(
        &mut self, 
        receiver_id: ValidAccountId, 
        amount: U128, 
        memo: Option<String>
    );
    fn ft_transfer_call(
        &mut self, 
        receiver_id: ValidAccountId, 
        amount: U128, 
        memo: Option<String>, 
        msg: String
    ) -> PromiseOrValue<U128>;
    fn ft_total_supply(&self) -> U128;
    fn ft_balance_of(&self, account_id: ValidAccountId) -> U128;

    fn burn(&mut self, amount: U128);
    fn near_withdraw(&mut self, amount: U128) -> Promise;
    fn near_deposit(&mut self);
}

#[ext_contract(ext_self)]
pub trait TokenPostActions {
    fn callback_get_total_supply(total_supply: U128);
}

#[near_bindgen]
impl Contract {
    pub fn callback_get_total_supply(mut _result: U128) {
        assert_self();
        assert_eq!(env::promise_results_count(), 1);
    
        match env::promise_result(0) {
            PromiseResult::NotReady => unreachable!(),
            PromiseResult::Failed => env::panic(b"failed to get total supply"),
            PromiseResult::Successful(result) => _result = U128::try_from_slice(&result).unwrap(),
        }
    }
}
