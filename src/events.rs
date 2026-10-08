use soroban_sdk::{symbol_short, Address, BytesN, Env, String};

pub fn emit_operator_registered(e: &Env, operator: Address, payout: Address, name: String) {
    let topics = (symbol_short!("op_reg"), operator);
    e.events().publish(topics, (payout, name));
}

pub fn emit_operator_status(e: &Env, operator: Address, active: bool) {
    let topics = (symbol_short!("op_stat"), operator);
    e.events().publish(topics, active);
}

pub fn emit_plan_created(e: &Env, plan_id: u32, operator: Address, daily_rate: i128, total_price: i128) {
    let topics = (symbol_short!("plan_new"), plan_id, operator);
    e.events().publish(topics, (daily_rate, total_price));
}

pub fn emit_plan_status(e: &Env, plan_id: u32, active: bool) {
    let topics = (symbol_short!("plan_act"), plan_id);
    e.events().publish(topics, active);
}

pub fn emit_lease_created(e: &Env, lease_id: u64, customer: Address, operator: Address, device_id: BytesN<32>, pool_id: Option<u64>) {
    let topics = (symbol_short!("lease_new"), lease_id, customer);
    e.events().publish(topics, (operator, device_id, pool_id));
}

pub fn emit_payment(
    e: &Env,
    lease_id: u64,
    payer: Address,
    amount: i128,
    paid_until: u64,
    total_paid: i128,
    operator_share: i128,
    pool_share: i128,
) {
    let topics = (symbol_short!("pay"), lease_id, payer);
    e.events().publish(topics, (amount, paid_until, total_paid, operator_share, pool_share));
}

pub fn emit_credit_granted(e: &Env, lease_id: u64, operator: Address, days: u32, new_paid_until: u64, reason: String) {
    let topics = (symbol_short!("credit"), lease_id, operator);
    e.events().publish(topics, (days, new_paid_until, reason));
}

pub fn emit_suspended(e: &Env, lease_id: u64, operator: Address, suspended: bool, reason: String) {
    let topics = (symbol_short!("suspend"), lease_id, operator);
    e.events().publish(topics, (suspended, reason));
}

pub fn emit_repossessed(e: &Env, lease_id: u64, operator: Address, reason: String) {
    let topics = (symbol_short!("repo"), lease_id, operator);
    e.events().publish(topics, reason);
}

pub fn emit_owned(e: &Env, lease_id: u64, customer: Address, total_paid: i128) {
    let topics = (symbol_short!("owned"), lease_id, customer);
    e.events().publish(topics, total_paid);
}

pub fn emit_emergency_pause(e: &Env, lease_id: u64, operator: Address, paused_until: u64, reason: String) {
    let topics = (symbol_short!("pause"), lease_id, operator);
    e.events().publish(topics, (paused_until, reason));
}

pub fn emit_plan_change_requested(e: &Env, lease_id: u64, current_plan: u32, new_plan_id: u32) {
    let topics = (symbol_short!("pl_req"), lease_id);
    e.events().publish(topics, (current_plan, new_plan_id));
}

pub fn emit_plan_change_accepted(e: &Env, lease_id: u64, old_plan: u32, new_plan: u32) {
    let topics = (symbol_short!("pl_acc"), lease_id);
    e.events().publish(topics, (old_plan, new_plan));
}

pub fn emit_device_swapped(e: &Env, lease_id: u64, operator: Address, old_device: BytesN<32>, new_device: BytesN<32>, reason: String) {
    let topics = (symbol_short!("swap"), lease_id, operator);
    e.events().publish(topics, (old_device, new_device, reason));
}

pub fn emit_lease_transferred(e: &Env, lease_id: u64, old_customer: Address, new_customer: Address) {
    let topics = (symbol_short!("xfer"), lease_id, old_customer);
    e.events().publish(topics, new_customer);
}

pub fn emit_pool_created(e: &Env, pool_id: u64, operator: Address, target_amount: i128, repayment_bps: u32) {
    let topics = (symbol_short!("pool_new"), pool_id, operator);
    e.events().publish(topics, (target_amount, repayment_bps));
}

pub fn emit_pool_funded(e: &Env, pool_id: u64, financier: Address, amount: i128, total_pool_funded: i128) {
    let topics = (symbol_short!("pool_fnd"), pool_id, financier);
    e.events().publish(topics, (amount, total_pool_funded));
}

pub fn emit_pool_claimed(e: &Env, pool_id: u64, financier: Address, claimed_amount: i128) {
    let topics = (symbol_short!("pool_clm"), pool_id, financier);
    e.events().publish(topics, claimed_amount);
}
