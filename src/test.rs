#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

fn setup_env() -> (Env, SunkeyPaygContractClient<'static>, Address) {
    let e = Env::default();
    e.mock_all_auths();

    let contract_id = e.register(SunkeyPaygContract, ());
    let client = SunkeyPaygContractClient::new(&e, &contract_id);

    let admin = Address::generate(&e);
    client.initialize(&admin);

    (e, client, admin)
}

#[test]
fn test_initialize_and_admin() {
    let (e, client, admin) = setup_env();
    let new_admin = Address::generate(&e);

    assert_eq!(client.get_admin(), admin);

    // Re-initialize should fail
    let res = client.try_initialize(&admin);
    assert_eq!(res.unwrap_err().unwrap(), Error::AlreadyInitialized);

    // Switch admin with auth
    client.set_admin(&new_admin);
    assert_eq!(client.get_admin(), new_admin);
}

#[test]
fn test_operator_lifecycle() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    let op_name = String::from_str(&e, "SolarCorp Africa");

    // Register operator
    client.register_operator(&operator, &op_name, &payout);
    let op = client.get_operator(&operator);
    assert_eq!(op.address, operator);
    assert_eq!(op.payout_address, payout);
    assert!(op.active);
    assert_eq!(op.total_leases, 0);
    assert_eq!(op.total_volume_collected, 0);

    // Duplicate registration fails
    let res = client.try_register_operator(&operator, &op_name, &payout);
    assert_eq!(res.unwrap_err().unwrap(), Error::OperatorAlreadyExists);

    // Update payout address
    let new_payout = Address::generate(&e);
    client.update_operator_payout(&operator, &new_payout);
    assert_eq!(client.get_operator(&operator).payout_address, new_payout);

    // Toggle active state
    client.set_operator_active(&operator, &false);
    assert!(!client.get_operator(&operator).active);

    client.set_operator_active(&operator, &true);
    assert!(client.get_operator(&operator).active);
}

#[test]
fn test_plan_lifecycle() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_name = String::from_str(&e, "50W Solar Home Kit");

    // Create plan: 2.5 token daily, 500 total, 50 deposit, 5 min pay, 7 days grace (604800s)
    let plan_id = 101;
    client.create_plan(
        &operator,
        &plan_id,
        &plan_name,
        &token,
        &2_500_000,
        &500_000_000,
        &50_000_000,
        &5_000_000,
        &604_800,
    );

    let plan = client.get_plan(&plan_id);
    assert_eq!(plan.plan_id, plan_id);
    assert_eq!(plan.operator, operator);
    assert_eq!(plan.daily_rate, 2_500_000);
    assert_eq!(plan.total_price, 500_000_000);
    assert_eq!(plan.deposit_amount, 50_000_000);
    assert!(plan.active);

    // Duplicate plan fails
    let res = client.try_create_plan(
        &operator,
        &plan_id,
        &plan_name,
        &token,
        &2_500_000,
        &500_000_000,
        &50_000_000,
        &5_000_000,
        &604_800,
    );
    assert_eq!(res.unwrap_err().unwrap(), Error::PlanAlreadyExists);

    // Invalid parameters fail
    let res_invalid = client.try_create_plan(
        &operator,
        &102,
        &plan_name,
        &token,
        &0, // daily_rate 0
        &500_000_000,
        &50_000_000,
        &5_000_000,
        &604_800,
    );
    assert_eq!(res_invalid.unwrap_err().unwrap(), Error::InvalidPlanParams);

    // Toggle plan status
    client.set_plan_active(&operator, &plan_id, &false);
    assert!(!client.get_plan(&plan_id).active);
    client.set_plan_active(&operator, &plan_id, &true);
    assert!(client.get_plan(&plan_id).active);
}
