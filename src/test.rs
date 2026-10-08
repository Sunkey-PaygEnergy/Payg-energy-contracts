#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{Client as TokenClient, StellarAssetClient},
    Address, BytesN, Env, String, Vec,
};

fn create_token<'a>(e: &Env, admin: &Address) -> (Address, TokenClient<'a>, StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    let token = sac.address();
    let token_client = TokenClient::new(e, &token);
    let asset_client = StellarAssetClient::new(e, &token);
    (token, token_client, asset_client)
}

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

#[test]
fn test_lease_creation_and_access() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 1;
    // Plan with 0 deposit
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Zero Deposit Kit"),
        &token,
        &2_000_000,
        &400_000_000,
        &0,
        &2_000_000,
        &86_400, // 1 day grace
    );

    let customer = Address::generate(&e);
    let device_id = BytesN::from_array(&e, &[1u8; 32]);
    let lease_id = 1001;

    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &None);

    let lease = client.get_lease(&lease_id);
    assert_eq!(lease.customer, customer);
    assert_eq!(lease.status, LeaseStatus::Active);
    assert_eq!(lease.total_paid, 0);
    assert!(client.is_device_assigned(&device_id));
    assert_eq!(client.get_device_lease(&device_id), lease_id);

    // Duplicate device fails
    let res_dup_device = client.try_create_lease(&operator, &1002, &customer, &plan_id, &device_id, &None);
    assert_eq!(res_dup_device.unwrap_err().unwrap(), Error::DeviceAlreadyAssigned);

    // Access check: at current timestamp
    let access = client.get_access(&lease_id);
    assert_eq!(access.lease_id, lease_id);
    assert_eq!(access.state, AccessState::Active);
    assert!(access.is_unlocked);
    assert_eq!(access.remaining_to_own, 400_000_000);
}

#[test]
fn test_batch_lease_creation() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 2;
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Community Plan"),
        &token,
        &1_000_000,
        &100_000_000,
        &0,
        &1_000_000,
        &86_400,
    );

    let mut batch = Vec::new(&e);
    for i in 1..=5 {
        let mut dev_bytes = [0u8; 32];
        dev_bytes[0] = i as u8;
        let device_id = BytesN::from_array(&e, &dev_bytes);
        batch.push_back(BatchLeaseInput {
            lease_id: 2000 + i as u64,
            customer: Address::generate(&e),
            plan_id,
            device_id,
            pool_id: None,
        });
    }

    let created_count = client.batch_create_leases(&operator, &batch);
    assert_eq!(created_count, 5);
    assert_eq!(client.get_operator(&operator).total_leases, 5);

    for i in 1..=5 {
        let mut dev_bytes = [0u8; 32];
        dev_bytes[0] = i as u8;
        let device_id = BytesN::from_array(&e, &dev_bytes);
        assert!(client.is_device_assigned(&device_id));
        let l = client.get_lease(&(2000 + i as u64));
        assert_eq!(l.status, LeaseStatus::Active);
    }
}

#[test]
fn test_payments_and_overpayment_capping() {
    let (e, client, admin) = setup_env();

    let (token, _token_client, asset_client) = create_token(&e, &admin);

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let plan_id = 10;
    // Daily rate = 5 token, total = 20 token, deposit = 5 token, min_payment = 2 token
    let daily_rate = 5_000_000;
    let total_price = 20_000_000;
    let deposit_amount = 5_000_000;
    let min_payment = 2_000_000;

    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Tier 1 Solar"),
        &token,
        &daily_rate,
        &total_price,
        &deposit_amount,
        &min_payment,
        &86_400,
    );

    let customer = Address::generate(&e);
    let relative = Address::generate(&e);

    // Mint tokens
    asset_client.mint(&customer, &100_000_000);
    asset_client.mint(&relative, &100_000_000);

    let lease_id = 999;
    let device_id = BytesN::from_array(&e, &[7u8; 32]);

    // Create lease with 5 token deposit -> buys 1 day (86400s)
    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &None);

    let lease_after_dep = client.get_lease(&lease_id);
    assert_eq!(lease_after_dep.total_paid, deposit_amount);
    let access1 = client.get_access(&lease_id);
    assert_eq!(access1.state, AccessState::Active);
    assert!(access1.seconds_remaining >= 86_399);

    // Relative makes top-up of 5 token (buys another day)
    let paid_relative = client.pay(&relative, &lease_id, &5_000_000);
    assert_eq!(paid_relative, 5_000_000);

    let lease_after_rel = client.get_lease(&lease_id);
    assert_eq!(lease_after_rel.total_paid, 10_000_000);

    // Overpayment capping test:
    // Total price is 20_000_000. Customer has paid 10_000_000.
    // Remaining to own is exactly 10_000_000.
    // Customer attempts to pay 15_000_000 (exceeds balance by 5_000_000).
    let paid_capped = client.pay(&customer, &lease_id, &15_000_000);

    // Only 10_000_000 should be charged!
    assert_eq!(paid_capped, 10_000_000);

    let lease_final = client.get_lease(&lease_id);
    assert_eq!(lease_final.total_paid, total_price);
    assert_eq!(lease_final.status, LeaseStatus::Owned);
    assert_eq!(lease_final.paid_until, u64::MAX);

    let access_final = client.get_access(&lease_id);
    assert_eq!(access_final.state, AccessState::Owned);
    assert!(access_final.is_unlocked);
    assert_eq!(access_final.remaining_to_own, 0);

    // Attempting further payment should fail
    let res_after_owned = client.try_pay(&customer, &lease_id, &5_000_000);
    assert_eq!(res_after_owned.unwrap_err().unwrap(), Error::LeaseAlreadyOwned);
}

#[test]
fn test_grant_credit() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 11;
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Promo Plan"),
        &token,
        &2_000_000,
        &100_000_000,
        &0,
        &1_000_000,
        &86_400,
    );

    let customer = Address::generate(&e);
    let device_id = BytesN::from_array(&e, &[8u8; 32]);
    let lease_id = 888;
    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &None);

    let current_time = e.ledger().timestamp();

    // Grant 3 promotional days of energy
    let new_paid_until = client.grant_credit(
        &operator,
        &lease_id,
        &3,
        &String::from_str(&e, "Welcome promotion"),
    );

    assert_eq!(new_paid_until, current_time + (3 * SECONDS_PER_DAY));
    let lease = client.get_lease(&lease_id);
    assert_eq!(lease.paid_until, new_paid_until);

    let access = client.get_access(&lease_id);
    assert_eq!(access.state, AccessState::Active);
    assert!(access.is_unlocked);
}

#[test]
fn test_financier_pool_and_pro_rata_splits() {
    let (e, client, admin) = setup_env();

    let (token, token_client, asset_client) = create_token(&e, &admin);

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let pool_id = 501;
    let target_amount = 10_000_000; // 10 token target
    let repayment_bps = 7_000;       // 70% of lease payments

    // Operator creates financier pool
    client.create_pool(
        &operator,
        &pool_id,
        &token,
        &String::from_str(&e, "Solar Batch Alpha Pool"),
        &target_amount,
        &repayment_bps,
    );

    let financier_a = Address::generate(&e);
    let financier_b = Address::generate(&e);

    asset_client.mint(&financier_a, &100_000_000);
    asset_client.mint(&financier_b, &100_000_000);

    // Financier A deposits 6,000,000 (60%)
    client.fund_pool(&financier_a, &pool_id, &6_000_000);
    // Financier B deposits 4,000,000 (40%)
    client.fund_pool(&financier_b, &pool_id, &4_000_000);

    let pool = client.get_pool(&pool_id);
    assert_eq!(pool.funded_amount, 10_000_000);
    assert!(pool.is_closed); // target reached

    // Target exceeded should fail
    let res_exceed = client.try_fund_pool(&financier_a, &pool_id, &1_000_000);
    assert!(res_exceed.is_err());

    // Create plan linked to pool
    let plan_id = 50;
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Pooled Solar Kit"),
        &token,
        &5_000_000,
        &50_000_000,
        &0,
        &1_000_000,
        &86_400,
    );

    let customer = Address::generate(&e);
    asset_client.mint(&customer, &100_000_000);

    let lease_id = 5555;
    let device_id = BytesN::from_array(&e, &[9u8; 32]);
    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &Some(pool_id));

    // Customer makes payment of 1,000,000
    // 70% = 700,000 goes to pool!
    // 30% = 300,000 goes to operator payout!
    let op_bal_before = token_client.balance(&payout);
    client.pay(&customer, &lease_id, &1_000_000);
    let op_bal_after = token_client.balance(&payout);
    assert_eq!(op_bal_after - op_bal_before, 300_000);

    // Check claimable earnings:
    // Financier A has 60% of 700,000 = 420,000
    // Financier B has 40% of 700,000 = 280,000
    let claimable_a = client.get_claimable_pool_earnings(&pool_id, &financier_a);
    let claimable_b = client.get_claimable_pool_earnings(&pool_id, &financier_b);
    assert_eq!(claimable_a, 420_000);
    assert_eq!(claimable_b, 280_000);

    // Financiers claim earnings
    let fa_bal_before = token_client.balance(&financier_a);
    let claimed_a = client.claim_pool_earnings(&financier_a, &pool_id);
    assert_eq!(claimed_a, 420_000);
    assert_eq!(token_client.balance(&financier_a) - fa_bal_before, 420_000);

    let fb_bal_before = token_client.balance(&financier_b);
    let claimed_b = client.claim_pool_earnings(&financier_b, &pool_id);
    assert_eq!(claimed_b, 280_000);
    assert_eq!(token_client.balance(&financier_b) - fb_bal_before, 280_000);

    // Second claim without new payments fails
    let res_no_earn = client.try_claim_pool_earnings(&financier_a, &pool_id);
    assert_eq!(res_no_earn.unwrap_err().unwrap(), Error::NoEarningsToClaim);
}

#[test]
fn test_emergency_pause_and_preservation() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 99;
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Test Plan"),
        &token,
        &1_000_000,
        &10_000_000,
        &0,
        &1_000_000,
        &86_400,
    );

    let customer = Address::generate(&e);
    let device_id = BytesN::from_array(&e, &[11u8; 32]);
    let lease_id = 7001;
    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &None);

    let current_time = e.ledger().timestamp();
    // Emergency pause for 10 days (864,000s)
    let pause_seconds = 864_000;
    let paused_until = client.emergency_pause(
        &operator,
        &lease_id,
        &pause_seconds,
        &String::from_str(&e, "Severe flooding flood relief"),
    );

    assert_eq!(paused_until, current_time + pause_seconds);
    let lease = client.get_lease(&lease_id);
    assert_eq!(lease.emergency_paused_until, paused_until);

    // Device remains active/unlocked during emergency pause
    assert!(client.is_active(&lease_id));
    let access = client.get_access(&lease_id);
    assert!(access.is_unlocked);
}

#[test]
fn test_plan_change_flow() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan1 = 1;
    let plan2 = 2;

    client.create_plan(
        &operator,
        &plan1,
        &String::from_str(&e, "Basic 20W"),
        &token,
        &1_000_000,
        &100_000_000,
        &0,
        &1_000_000,
        &86_400,
    );
    client.create_plan(
        &operator,
        &plan2,
        &String::from_str(&e, "Upgraded 50W"),
        &token,
        &2_000_000,
        &200_000_000,
        &0,
        &2_000_000,
        &86_400,
    );

    let customer = Address::generate(&e);
    let device_id = BytesN::from_array(&e, &[12u8; 32]);
    let lease_id = 7002;
    client.create_lease(&operator, &lease_id, &customer, &plan1, &device_id, &None);

    // Operator requests plan change
    client.request_plan_change(&operator, &lease_id, &plan2);
    assert_eq!(client.get_pending_plan_change(&lease_id), plan2);

    // Unauthorized non-customer fails
    let random_user = Address::generate(&e);
    let res_unauth = client.try_accept_plan_change(&random_user, &lease_id);
    assert_eq!(res_unauth.unwrap_err().unwrap(), Error::Unauthorized);

    // Customer accepts
    client.accept_plan_change(&customer, &lease_id);
    let updated_lease = client.get_lease(&lease_id);
    assert_eq!(updated_lease.plan_id, plan2);
    assert!(client.try_get_pending_plan_change(&lease_id).is_err());
}

#[test]
fn test_swap_and_transfer() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 3;
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Standard"),
        &token,
        &1_000_000,
        &100_000_000,
        &0,
        &1_000_000,
        &86_400,
    );

    let customer_a = Address::generate(&e);
    let customer_b = Address::generate(&e);
    let dev1 = BytesN::from_array(&e, &[13u8; 32]);
    let dev2 = BytesN::from_array(&e, &[14u8; 32]);
    let lease_id = 7003;

    client.create_lease(&operator, &lease_id, &customer_a, &plan_id, &dev1, &None);

    // Operator swaps hardware device to dev2
    client.device_swap(
        &operator,
        &lease_id,
        &dev2,
        &String::from_str(&e, "Panel upgrade maintenance"),
    );

    assert!(!client.is_device_assigned(&dev1)); // old freed
    assert!(client.is_device_assigned(&dev2));  // new assigned
    assert_eq!(client.get_lease(&lease_id).device_id, dev2);

    // Customer transfers lease to customer_b
    client.transfer_lease_customer(&customer_a, &customer_b, &lease_id);
    assert_eq!(client.get_lease(&lease_id).customer, customer_b);
}

#[test]
fn test_repossession_guard_rails() {
    let (e, client, _admin) = setup_env();

    let operator = Address::generate(&e);
    let payout = Address::generate(&e);
    client.register_operator(&operator, &String::from_str(&e, "SolarCorp"), &payout);

    let token = Address::generate(&e);
    let plan_id = 4;
    let grace_period = 86_400; // 1 day grace
    client.create_plan(
        &operator,
        &plan_id,
        &String::from_str(&e, "Rigid Lease"),
        &token,
        &1_000_000,
        &50_000_000,
        &0,
        &1_000_000,
        &grace_period,
    );

    let customer = Address::generate(&e);
    let device_id = BytesN::from_array(&e, &[15u8; 32]);
    let lease_id = 7004;

    client.create_lease(&operator, &lease_id, &customer, &plan_id, &device_id, &None);

    // Grant 2 days of credit
    client.grant_credit(
        &operator,
        &lease_id,
        &2,
        &String::from_str(&e, "Initial credit"),
    );

    let t0 = e.ledger().timestamp();

    // Case 1: Attempt repossession while paid_until > current_time
    let res_active = client.try_repossess(&operator, &lease_id, &String::from_str(&e, "premature repo"));
    assert_eq!(res_active.unwrap_err().unwrap(), Error::RepossessionNotAllowed);

    // Advance time past paid_until (2 days) + 12 hours (within 1 day grace period)
    e.ledger().set_timestamp(t0 + (2 * 86_400) + 43_200);

    // Access status shows GracePeriod
    let access = client.get_access(&lease_id);
    assert_eq!(access.state, AccessState::GracePeriod);
    assert!(!access.is_repossessable);

    // Case 2: Attempt repossession during active grace period
    let res_grace = client.try_repossess(&operator, &lease_id, &String::from_str(&e, "repo in grace"));
    assert_eq!(res_grace.unwrap_err().unwrap(), Error::GracePeriodActive);

    // Advance time past grace period (past default deadline)
    e.ledger().set_timestamp(t0 + (2 * 86_400) + grace_period + 10);

    let access_locked = client.get_access(&lease_id);
    assert_eq!(access_locked.state, AccessState::Locked);
    assert!(access_locked.is_repossessable);

    // Case 3: Repossession now allowed!
    client.repossess(&operator, &lease_id, &String::from_str(&e, "Defaulted beyond grace"));

    let lease_repo = client.get_lease(&lease_id);
    assert_eq!(lease_repo.status, LeaseStatus::Repossessed);
    assert!(!client.is_device_assigned(&device_id)); // device recovered
}
