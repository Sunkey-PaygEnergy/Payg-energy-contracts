#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, Address, Env, String};
pub use errors::Error;
pub use types::*;

#[contract]
pub struct SunkeyPaygContract;

#[contractimpl]
impl SunkeyPaygContract {
    pub fn initialize(e: Env, admin: Address) -> Result<(), Error> {
        if storage::get_admin(&e).is_some() {
            return Err(Error::AlreadyInitialized);
        }
        storage::set_admin(&e, &admin);
        Ok(())
    }

    pub fn set_admin(e: Env, new_admin: Address) -> Result<(), Error> {
        let current_admin = storage::get_admin(&e).ok_or(Error::NotInitialized)?;
        current_admin.require_auth();
        storage::set_admin(&e, &new_admin);
        Ok(())
    }

    pub fn get_admin(e: Env) -> Result<Address, Error> {
        storage::get_admin(&e).ok_or(Error::NotInitialized)
    }

    pub fn register_operator(
        e: Env,
        operator: Address,
        name: String,
        payout_address: Address,
    ) -> Result<(), Error> {
        let admin = storage::get_admin(&e).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        if storage::get_operator(&e, &operator).is_some() {
            return Err(Error::OperatorAlreadyExists);
        }

        let op = Operator {
            address: operator.clone(),
            name: name.clone(),
            payout_address: payout_address.clone(),
            active: true,
            registered_at: e.ledger().timestamp(),
            total_leases: 0,
            total_volume_collected: 0,
        };

        storage::set_operator(&e, &op);
        events::emit_operator_registered(&e, operator, payout_address, name);
        Ok(())
    }

    pub fn set_operator_active(e: Env, operator: Address, active: bool) -> Result<(), Error> {
        let admin = storage::get_admin(&e).ok_or(Error::NotInitialized)?;
        admin.require_auth();

        let mut op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        op.active = active;
        storage::set_operator(&e, &op);
        events::emit_operator_status(&e, operator, active);
        Ok(())
    }

    pub fn update_operator_payout(e: Env, operator: Address, new_payout: Address) -> Result<(), Error> {
        operator.require_auth();
        let mut op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        op.payout_address = new_payout;
        storage::set_operator(&e, &op);
        Ok(())
    }

    pub fn get_operator(e: Env, operator: Address) -> Result<Operator, Error> {
        storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)
    }

    pub fn create_plan(
        e: Env,
        operator: Address,
        plan_id: u32,
        name: String,
        token: Address,
        daily_rate: i128,
        total_price: i128,
        deposit_amount: i128,
        min_payment: i128,
        grace_period_seconds: u64,
    ) -> Result<(), Error> {
        operator.require_auth();

        let op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        if !op.active {
            return Err(Error::OperatorInactive);
        }

        if daily_rate <= 0 || total_price <= 0 || min_payment <= 0 || deposit_amount < 0 {
            return Err(Error::InvalidPlanParams);
        }
        if min_payment > total_price || deposit_amount > total_price {
            return Err(Error::InvalidPlanParams);
        }
        if storage::get_plan(&e, plan_id).is_some() {
            return Err(Error::PlanAlreadyExists);
        }

        let plan = Plan {
            plan_id,
            operator: operator.clone(),
            name,
            token,
            daily_rate,
            total_price,
            deposit_amount,
            min_payment,
            grace_period_seconds,
            active: true,
            created_at: e.ledger().timestamp(),
        };

        storage::set_plan(&e, &plan);
        events::emit_plan_created(&e, plan_id, operator, daily_rate, total_price);
        Ok(())
    }

    pub fn set_plan_active(e: Env, operator: Address, plan_id: u32, active: bool) -> Result<(), Error> {
        operator.require_auth();

        let mut plan = storage::get_plan(&e, plan_id).ok_or(Error::PlanNotFound)?;
        if plan.operator != operator {
            return Err(Error::Unauthorized);
        }

        plan.active = active;
        storage::set_plan(&e, &plan);
        events::emit_plan_status(&e, plan_id, active);
        Ok(())
    }

    pub fn get_plan(e: Env, plan_id: u32) -> Result<Plan, Error> {
        storage::get_plan(&e, plan_id).ok_or(Error::PlanNotFound)
    }

    pub fn version(_e: Env) -> u32 {
        1
    }
}
