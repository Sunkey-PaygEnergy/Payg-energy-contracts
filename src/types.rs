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

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum LeaseStatus {
    Active = 1,
    Suspended = 2,
    Repossessed = 3,
    Owned = 4,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Lease {
    pub lease_id: u64,
    pub operator: Address,
    pub customer: Address,
    pub plan_id: u32,
    pub device_id: BytesN<32>,
    pub pool_id: Option<u64>,
    pub status: LeaseStatus,
    pub paid_until: u64,
    pub total_paid: i128,
    pub emergency_paused_until: u64,
    pub created_at: u64,
    pub last_payment_at: u64,
}
