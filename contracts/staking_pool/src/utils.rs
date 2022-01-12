use crate::*;

/// Interface for a voting contract.
#[ext_contract(ext_voting)]
pub trait VoteContract {
    /// Method for validators to vote or withdraw the vote.
    /// Votes for if `is_vote` is true, or withdraws the vote if `is_vote` is false.
    fn vote(&mut self, is_vote: bool);
}

/// Interface for bNear contract.
#[ext_contract(ext_fungible_token)]
pub trait FungibleToken {
    /// Method for staking pool to mint bNEAR to the delegators when they stakes.
    fn mint(&mut self, account_id: AccountId, amount: U128);
    /// Method for staking pool to burn bNEAR from the delegators when they unstakes.
    fn burn(&mut self, account_id: AccountId, amount: U128);
}

/// Interface for the contract itself.
#[ext_contract(ext_self)]
pub trait SelfContract {
    /// A callback to check the result of the staking action.
    /// In case the stake amount is less than the minimum staking threshold, the staking action
    /// fails, and the stake amount is not changed. This might lead to inconsistent state and the
    /// follow withdraw calls might fail. To mitigate this, the contract will issue a new unstaking
    /// action in case of the failure of the first staking action.
    fn on_stake_action(&mut self);

    /// A callback to check the result of the minting action.
    /// In case the account register action couldn't completed due to such as storage staking, the minting
    /// actiong and the stake action will revert.
    fn on_mint_action(&mut self);

    /// A callback to check the result of the burning action.
    /// In case the balance of bNEAR is less than unstaking amount, the burning action and
    /// the unstake action will revert.
    fn on_burn_action(&mut self);
}

#[near_bindgen]
impl StakingContract {
    /*************/
    /* Callbacks */
    /*************/
    #[private]
    pub fn on_stake_action(&mut self) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );
        let stake_action_succeeded = match env::promise_result(0) {
            PromiseResult::Successful(_) => true,
            _ => false,
        };

        // If the stake action failed and the current locked amount is positive, then the contract
        // has to unstake.
        if !stake_action_succeeded && env::account_locked_balance() > 0 {
            Promise::new(env::current_account_id()).stake(0, self.stake_public_key.clone());
        }
    }

    #[private]
    pub fn on_mint_action(&mut self) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );

        if let PromiseResult::Failed = env::promise_result(0) {
            env::panic(b"Minting action failed.");
        }
    }

    #[private]
    pub fn on_burn_action(&mut self) {
        assert_eq!(
            env::promise_results_count(),
            1,
            "Contract expected a result on the callback"
        );

        if let PromiseResult::Failed = env::promise_result(0) {
            env::panic(b"Burning action failed.");
        }
    }
}