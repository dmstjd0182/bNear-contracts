use crate::*;

#[near_bindgen]
impl Contract {
    /// Staking pool's method.
    /// Mint bNear corresponding to staked NEAR.
    pub fn mint(&mut self, account_id: AccountId, amount: U128) {
        self.assert_staking_pool();

        let amount: Balance = amount.0;
        assert!(amount > 0, "Requires positive attached deposit");
        if !self.ft.accounts.contains_key(&account_id) {
            // Not registered
            self.ft.internal_register_account(&account_id);
        }
        self.ft.internal_deposit(&account_id, amount);
        log!("Deposit {} NEAR to {}", amount, account_id);
    }

    // Transfer Near to unbonded account
    #[payable]
    pub fn near_withdraw(&mut self, amount: U128) -> Promise {
        self.assert_staking_pool();

        let account_id = env::signer_account_id();
        let amount: Balance = amount.into();
        log!("Withdraw {} NEAR from {}", amount, account_id);
        // Transferring NEAR and refunding 1 yoctoNEAR.
        Promise::new(account_id).transfer(amount + 1)
    }

    // waiting unbonding
    // Reduce total_supply and account's bNear
    pub fn burn(&mut self, amount: U128) {
        self.assert_staking_pool();

        let account_id = env::signer_account_id();
        let amount: Balance = amount.into();
        self.ft.internal_withdraw(&account_id, amount);
        log!("Burned {} bNEAR from {}", amount, account_id);
    }
}
