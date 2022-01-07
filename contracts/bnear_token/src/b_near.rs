use crate::*;
use near_sdk::json_types::U128;
use near_sdk::{assert_one_yocto, env, log, Promise, Balance};

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn near_deposit(&mut self) {
        let mut amount: Balance = env::attached_deposit();
        assert!(amount > 0, "Requires positive attached deposit");
        let account_id = env::signer_account_id();
        if !self.ft.accounts.contains_key(&account_id) {
            // Not registered, register if enough $NEAR has been attached.
            // Subtract registration amount from the account balance.
            assert!(
                amount >= self.ft.storage_balance_bounds().min.0,
                "ERR_DEPOSIT_TOO_SMALL"
            );
            self.ft.internal_register_account(&account_id);
            amount -= self.ft.storage_balance_bounds().min.0;
        }
        self.ft.internal_deposit(&account_id, amount);
        log!("Deposit {} NEAR to {}", amount, account_id);
    }

    // Transfer Near to unbonded account
    #[payable]
    pub fn near_withdraw(&mut self, amount: U128) -> Promise {
        assert_one_yocto();
        let account_id = env::signer_account_id();
        let amount: Balance = amount.into();
        log!("Withdraw {} NEAR from {}", amount, account_id);
        // Transferring NEAR and refunding 1 yoctoNEAR.
        Promise::new(account_id).transfer(amount + 1)
    }

    // waiting unbonding
    // Reduce total_supply and account's bNear
    pub fn burn(&mut self, amount: U128) {
        let account_id = env::signer_account_id();
        let amount: Balance = amount.into();
        self.ft.internal_withdraw(&account_id, amount);
        log!("Burned {} bNEAR from {}", amount, account_id);
    }
}
