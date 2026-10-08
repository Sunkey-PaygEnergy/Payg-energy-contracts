use soroban_sdk::{contracttype, Address, BytesN};

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
