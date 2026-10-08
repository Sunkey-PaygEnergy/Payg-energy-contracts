use soroban_sdk::{contracttype, Address, BytesN, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    // Instance data keys
    Admin,
    OperatorCount,
    PlanCount,
    LeaseCount,
    PoolCount,

    // Persistent data keys (Fleet scale)
    Operator(Address),
    Plan(u32),
    Lease(u64),
    Device(BytesN<32>),
    Pool(u64),
    PoolFinancier(u64, Address),
    PendingPlanChange(u64),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Operator {
    pub address: Address,
    pub name: String,
    pub payout_address: Address,
    pub active: bool,
    pub registered_at: u64,
    pub total_leases: u64,
    pub total_volume_collected: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    pub plan_id: u32,
    pub operator: Address,
    pub name: String,
    pub token: Address,
    pub daily_rate: i128,
    pub total_price: i128,
    pub deposit_amount: i128,
    pub min_payment: i128,
    pub grace_period_seconds: u64,
    pub active: bool,
    pub created_at: u64,
}
