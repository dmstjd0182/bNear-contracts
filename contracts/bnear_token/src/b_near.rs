use crate::*;

#[near_bindgen]
impl Contract {
    /// Staking pool's method.
    /// Mint bNEAR corresponding to staked NEAR.
    pub fn mint(&mut self, account_id: AccountId, amount: Balance) {
        self.assert_staking_pool();

        assert!(amount > 0, "Requires positive attached deposit");
        if !self.ft.accounts.contains_key(&account_id) {
            // Not registered
            self.ft.internal_register_account(&account_id);
        }
        self.ft.internal_deposit(&account_id, amount);
        log!("Minted {} bNEAR to {}", amount, account_id);
    }

    /// Staking pool's method.
    /// Reduce total_supply and account's bNEAR
    pub fn burn(&mut self, account_id: AccountId, amount: Balance) {
        self.assert_staking_pool();

        self.ft.internal_withdraw(&account_id, amount);
        log!("Burned {} bNEAR from {}", amount, account_id);
    }
}
