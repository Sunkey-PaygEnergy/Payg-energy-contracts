use soroban_sdk::{Address, BytesN, Env};
use crate::types::{DataKey, FinancierPool, Lease, Operator, Plan, PoolFinancier};

pub const INSTANCE_LIFETIME_THRESHOLD: u32 = 50_000;
pub const INSTANCE_BUMP_AMOUNT: u32 = 100_000;

pub const PERSISTENT_LIFETIME_THRESHOLD: u32 = 100_000;
pub const PERSISTENT_BUMP_AMOUNT: u32 = 500_000;

pub fn bump_instance(e: &Env) {
    e.storage().instance().extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
}

pub fn get_admin(e: &Env) -> Option<Address> {
    bump_instance(e);
    e.storage().instance().get(&DataKey::Admin)
}

pub fn set_admin(e: &Env, admin: &Address) {
    bump_instance(e);
    e.storage().instance().set(&DataKey::Admin, admin);
}

pub fn get_operator(e: &Env, operator: &Address) -> Option<Operator> {
    let key = DataKey::Operator(operator.clone());
    if let Some(op) = e.storage().persistent().get::<DataKey, Operator>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(op)
    } else {
        None
    }
}

pub fn set_operator(e: &Env, op: &Operator) {
    let key = DataKey::Operator(op.address.clone());
    e.storage().persistent().set(&key, op);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_plan(e: &Env, plan_id: u32) -> Option<Plan> {
    let key = DataKey::Plan(plan_id);
    if let Some(plan) = e.storage().persistent().get::<DataKey, Plan>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(plan)
    } else {
        None
    }
}

pub fn set_plan(e: &Env, plan: &Plan) {
    let key = DataKey::Plan(plan.plan_id);
    e.storage().persistent().set(&key, plan);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_lease(e: &Env, lease_id: u64) -> Option<Lease> {
    let key = DataKey::Lease(lease_id);
    if let Some(lease) = e.storage().persistent().get::<DataKey, Lease>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(lease)
    } else {
        None
    }
}

pub fn set_lease(e: &Env, lease: &Lease) {
    let key = DataKey::Lease(lease.lease_id);
    e.storage().persistent().set(&key, lease);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_device_lease(e: &Env, device_id: &BytesN<32>) -> Option<u64> {
    let key = DataKey::Device(device_id.clone());
    if let Some(lease_id) = e.storage().persistent().get::<DataKey, u64>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(lease_id)
    } else {
        None
    }
}

pub fn set_device_lease(e: &Env, device_id: &BytesN<32>, lease_id: u64) {
    let key = DataKey::Device(device_id.clone());
    e.storage().persistent().set(&key, &lease_id);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn remove_device_lease(e: &Env, device_id: &BytesN<32>) {
    let key = DataKey::Device(device_id.clone());
    e.storage().persistent().remove(&key);
}

pub fn get_pool(e: &Env, pool_id: u64) -> Option<FinancierPool> {
    let key = DataKey::Pool(pool_id);
    if let Some(pool) = e.storage().persistent().get::<DataKey, FinancierPool>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(pool)
    } else {
        None
    }
}

pub fn set_pool(e: &Env, pool: &FinancierPool) {
    let key = DataKey::Pool(pool.pool_id);
    e.storage().persistent().set(&key, pool);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_pool_financier(e: &Env, pool_id: u64, financier: &Address) -> Option<PoolFinancier> {
    let key = DataKey::PoolFinancier(pool_id, financier.clone());
    if let Some(pos) = e.storage().persistent().get::<DataKey, PoolFinancier>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(pos)
    } else {
        None
    }
}

pub fn set_pool_financier(e: &Env, pos: &PoolFinancier) {
    let key = DataKey::PoolFinancier(pos.pool_id, pos.financier.clone());
    e.storage().persistent().set(&key, pos);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn get_pending_plan_change(e: &Env, lease_id: u64) -> Option<u32> {
    let key = DataKey::PendingPlanChange(lease_id);
    if let Some(plan_id) = e.storage().persistent().get::<DataKey, u32>(&key) {
        e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        Some(plan_id)
    } else {
        None
    }
}

pub fn set_pending_plan_change(e: &Env, lease_id: u64, new_plan_id: u32) {
    let key = DataKey::PendingPlanChange(lease_id);
    e.storage().persistent().set(&key, &new_plan_id);
    e.storage().persistent().extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
}

pub fn clear_pending_plan_change(e: &Env, lease_id: u64) {
    let key = DataKey::PendingPlanChange(lease_id);
    e.storage().persistent().remove(&key);
}
