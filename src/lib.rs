#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, Env, String};
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

    pub fn create_pool(
        e: Env,
        operator: Address,
        pool_id: u64,
        token: Address,
        name: String,
        target_amount: i128,
        repayment_bps: u32,
    ) -> Result<(), Error> {
        operator.require_auth();

        let op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        if !op.active {
            return Err(Error::OperatorInactive);
        }

        if target_amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if repayment_bps == 0 || repayment_bps > BPS_DENOMINATOR {
            return Err(Error::InvalidBasisPoints);
        }
        if storage::get_pool(&e, pool_id).is_some() {
            return Err(Error::PoolAlreadyExists);
        }

        let pool = FinancierPool {
            pool_id,
            operator,
            token,
            name,
            target_amount,
            funded_amount: 0,
            repayment_bps,
            total_repaid: 0,
            acc_reward_per_share: 0,
            is_closed: false,
            created_at: e.ledger().timestamp(),
        };

        storage::set_pool(&e, &pool);
        events::emit_pool_created(&e, pool_id, pool.operator, target_amount, repayment_bps);
        Ok(())
    }

    pub fn fund_pool(e: Env, financier: Address, pool_id: u64, amount: i128) -> Result<(), Error> {
        financier.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut pool = storage::get_pool(&e, pool_id).ok_or(Error::PoolNotFound)?;
        if pool.is_closed {
            return Err(Error::PoolFullyFunded);
        }
        if pool.funded_amount + amount > pool.target_amount {
            return Err(Error::PoolFundingTargetExceeded);
        }

        let op = storage::get_operator(&e, &pool.operator).ok_or(Error::OperatorNotFound)?;

        // Transfer funds from financier to operator's payout address to finance solar inventory
        let client = TokenClient::new(&e, &pool.token);
        client.transfer(&financier, &op.payout_address, &amount);

        // Update or create financier position
        let mut pos = storage::get_pool_financier(&e, pool_id, &financier).unwrap_or(PoolFinancier {
            pool_id,
            financier: financier.clone(),
            deposit_amount: 0,
            reward_debt: 0,
            claimed_amount: 0,
        });

        pos.deposit_amount += amount;
        pos.reward_debt += (amount * pool.acc_reward_per_share) / ACC_PRECISION;
        storage::set_pool_financier(&e, &pos);

        pool.funded_amount += amount;
        if pool.funded_amount == pool.target_amount {
            pool.is_closed = true;
        }
        storage::set_pool(&e, &pool);

        events::emit_pool_funded(&e, pool_id, financier, amount, pool.funded_amount);
        Ok(())
    }

    pub fn claim_pool_earnings(e: Env, financier: Address, pool_id: u64) -> Result<i128, Error> {
        financier.require_auth();

        let pool = storage::get_pool(&e, pool_id).ok_or(Error::PoolNotFound)?;
        let mut pos = storage::get_pool_financier(&e, pool_id, &financier).ok_or(Error::Unauthorized)?;

        let accumulated = (pos.deposit_amount * pool.acc_reward_per_share) / ACC_PRECISION;
        let claimable = accumulated - pos.reward_debt;
        if claimable <= 0 {
            return Err(Error::NoEarningsToClaim);
        }

        // Transfer claimable yield from contract reserve to financier
        let client = TokenClient::new(&e, &pool.token);
        client.transfer(&e.current_contract_address(), &financier, &claimable);

        pos.reward_debt = accumulated;
        pos.claimed_amount += claimable;
        storage::set_pool_financier(&e, &pos);

        events::emit_pool_claimed(&e, pool_id, financier, claimable);
        Ok(claimable)
    }

    pub fn get_claimable_pool_earnings(e: Env, pool_id: u64, financier: Address) -> Result<i128, Error> {
        let pool = storage::get_pool(&e, pool_id).ok_or(Error::PoolNotFound)?;
        let pos = storage::get_pool_financier(&e, pool_id, &financier).ok_or(Error::Unauthorized)?;

        let accumulated = (pos.deposit_amount * pool.acc_reward_per_share) / ACC_PRECISION;
        let claimable = accumulated - pos.reward_debt;
        if claimable > 0 {
            Ok(claimable)
        } else {
            Ok(0)
        }
    }

    pub fn get_pool(e: Env, pool_id: u64) -> Result<FinancierPool, Error> {
        storage::get_pool(&e, pool_id).ok_or(Error::PoolNotFound)
    }

    pub fn get_pool_financier(e: Env, pool_id: u64, financier: Address) -> Result<PoolFinancier, Error> {
        storage::get_pool_financier(&e, pool_id, &financier).ok_or(Error::Unauthorized)
    }

    pub fn version(_e: Env) -> u32 {
        1
    }
}
