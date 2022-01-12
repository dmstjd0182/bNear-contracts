use super::*;
use near_sdk::test_utils::{accounts, VMContextBuilder};
use near_sdk::{testing_env, MockedBlockchain};

/// accounts 0: staking_pool
/// accounts 1: signer(blockwave)
fn setup_contract() -> (VMContextBuilder, Contract) {
    let mut context = VMContextBuilder::new();
    testing_env!(context.predecessor_account_id(accounts(0)).build());
    let contract = Contract::new(accounts(0));
    (context, contract)
}

fn mint(
    context: &mut VMContextBuilder,
    contract: &mut Contract,
    account_id: ValidAccountId,
    amount: Balance,
) {
    testing_env!(context
        .predecessor_account_id(accounts(0))
        .build());
    
    contract.mint(account_id.clone().into(), amount);
    println!("Minted {} bNEAR to {}", amount, account_id.to_string());
}

fn burn(
    context: &mut VMContextBuilder,
    contract: &mut Contract,
    account_id: ValidAccountId,
    amount: Balance,
) {
    testing_env!(context
        .predecessor_account_id(accounts(0))
        .build());
    
    contract.burn(account_id.clone().into(), amount);
    println!("Burned {} bNEAR from {}", amount, account_id.to_string());
}

fn get_total_supply(
    contract: &mut Contract
) -> Balance {
    contract.ft.total_supply
}

fn get_balance_of(
    contract: &mut Contract,
    account_id: ValidAccountId,
) -> Balance {
    contract.ft.ft_balance_of(account_id).into()
}

#[test]
#[should_panic]
fn test_assert_staking_pool() {
    let (mut context, contract) = setup_contract();
    // predecessor is different from staking pool.
    testing_env!(context
        .predecessor_account_id(accounts(1))
        .build());
    // this should panic
    contract.assert_staking_pool();
}

#[test]
fn test_mint_and_burn() {
    let (mut context, mut contract) = setup_contract();

    let total_supply = get_total_supply(&mut contract);
    let balance = get_balance_of(&mut contract, accounts(1));
    assert_eq!(total_supply, 0);
    assert_eq!(balance, 0);

    // mint
    mint(&mut context, &mut contract, accounts(1), 100);
    mint(&mut context, &mut contract, accounts(2), 50);

    let total_supply = get_total_supply(&mut contract);
    let balance_1 = get_balance_of(&mut contract, accounts(1));
    let balance_2 = get_balance_of(&mut contract, accounts(2));
    assert_eq!(total_supply, 150);
    assert_eq!(balance_1, 100);
    assert_eq!(balance_2, 50);

    // burn
    burn(&mut context, &mut contract, accounts(1), 23);
    burn(&mut context, &mut contract, accounts(2), 11);

    let total_supply = get_total_supply(&mut contract);
    let balance_1 = get_balance_of(&mut contract, accounts(1));
    let balance_2 = get_balance_of(&mut contract, accounts(2));
    assert_eq!(total_supply, 116);
    assert_eq!(balance_1, 77);
    assert_eq!(balance_2, 39);
}