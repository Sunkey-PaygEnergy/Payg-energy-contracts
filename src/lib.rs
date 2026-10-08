#![no_std]

pub mod errors;
pub mod events;
pub mod storage;
pub mod types;

use soroban_sdk::{contract, contractimpl, token::TokenClient, Address, BytesN, Env, String, Vec};
pub use errors::Error;
pub use types::*;

pub const MAX_BATCH_SIZE: u32 = 50;

#[contract]
pub struct SunkeyPaygContract;

impl SunkeyPaygContract {
    fn internal_split_and_transfer(
        e: &Env,
        payer: &Address,
        token: &Address,
        amount: i128,
        operator: &Operator,
        pool_opt: &mut Option<FinancierPool>,
    ) -> (i128, i128) {
        let client = TokenClient::new(e, token);
        let mut pool_share: i128 = 0;
        let mut op_share: i128 = amount;

        if let Some(pool) = pool_opt {
            if pool.funded_amount > 0 && pool.repayment_bps > 0 && pool.total_repaid < pool.target_amount {
                let remaining_target = pool.target_amount - pool.total_repaid;
                let calculated_pool_share = (amount * (pool.repayment_bps as i128)) / (BPS_DENOMINATOR as i128);
                pool_share = if calculated_pool_share > remaining_target {
                    remaining_target
                } else {
                    calculated_pool_share
                };

                if pool_share > 0 {
                    op_share = amount - pool_share;
                    client.transfer(payer, &e.current_contract_address(), &pool_share);
                    let reward_inc = (pool_share * ACC_PRECISION) / pool.funded_amount;
                    pool.acc_reward_per_share += reward_inc;
                    pool.total_repaid += pool_share;

                    if pool.total_repaid >= pool.target_amount {
                        pool.is_closed = true;
                    }
                }
            }
        }

        if op_share > 0 {
            client.transfer(payer, &operator.payout_address, &op_share);
        }

        (op_share, pool_share)
    }
}

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

        let client = TokenClient::new(&e, &pool.token);
        client.transfer(&financier, &op.payout_address, &amount);

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

    pub fn create_lease(
        e: Env,
        operator: Address,
        lease_id: u64,
        customer: Address,
        plan_id: u32,
        device_id: BytesN<32>,
        pool_id: Option<u64>,
    ) -> Result<(), Error> {
        operator.require_auth();

        let mut op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        if !op.active {
            return Err(Error::OperatorInactive);
        }

        let plan = storage::get_plan(&e, plan_id).ok_or(Error::PlanNotFound)?;
        if !plan.active || plan.operator != operator {
            return Err(Error::PlanInactive);
        }

        if storage::get_lease(&e, lease_id).is_some() {
            return Err(Error::LeaseAlreadyExists);
        }

        if storage::get_device_lease(&e, &device_id).is_some() {
            return Err(Error::DeviceAlreadyAssigned);
        }

        let mut pool_opt: Option<FinancierPool> = None;
        if let Some(pid) = pool_id {
            let pool = storage::get_pool(&e, pid).ok_or(Error::PoolNotFound)?;
            if pool.operator != operator {
                return Err(Error::Unauthorized);
            }
            pool_opt = Some(pool);
        }

        let current_time = e.ledger().timestamp();
        let mut paid_until = current_time;
        let mut total_paid: i128 = 0;

        if plan.deposit_amount > 0 {
            customer.require_auth();
            let (op_share, pool_share) = Self::internal_split_and_transfer(
                &e,
                &customer,
                &plan.token,
                plan.deposit_amount,
                &op,
                &mut pool_opt,
            );

            let deposit_seconds = ((plan.deposit_amount * (SECONDS_PER_DAY as i128)) / plan.daily_rate) as u64;
            paid_until = current_time + deposit_seconds;
            total_paid = plan.deposit_amount;
            op.total_volume_collected += op_share;

            if let Some(ref p) = pool_opt {
                storage::set_pool(&e, p);
            }

            events::emit_payment(
                &e,
                lease_id,
                customer.clone(),
                plan.deposit_amount,
                paid_until,
                total_paid,
                op_share,
                pool_share,
            );
        }

        op.total_leases += 1;
        storage::set_operator(&e, &op);

        let is_owned = total_paid >= plan.total_price;
        let status = if is_owned {
            LeaseStatus::Owned
        } else {
            LeaseStatus::Active
        };

        let lease = Lease {
            lease_id,
            operator: operator.clone(),
            customer: customer.clone(),
            plan_id,
            device_id: device_id.clone(),
            pool_id,
            status,
            paid_until,
            total_paid,
            emergency_paused_until: 0,
            created_at: current_time,
            last_payment_at: current_time,
        };

        storage::set_lease(&e, &lease);
        storage::set_device_lease(&e, &device_id, lease_id);

        events::emit_lease_created(&e, lease_id, customer, operator, device_id, pool_id);
        if is_owned {
            events::emit_owned(&e, lease_id, lease.customer, total_paid);
        }

        Ok(())
    }

    pub fn batch_create_leases(
        e: Env,
        operator: Address,
        leases: Vec<BatchLeaseInput>,
    ) -> Result<u32, Error> {
        operator.require_auth();

        if leases.len() > MAX_BATCH_SIZE {
            return Err(Error::BatchSizeExceeded);
        }

        let mut op = storage::get_operator(&e, &operator).ok_or(Error::OperatorNotFound)?;
        if !op.active {
            return Err(Error::OperatorInactive);
        }

        let current_time = e.ledger().timestamp();
        let mut count: u32 = 0;

        for item in leases.iter() {
            let plan = storage::get_plan(&e, item.plan_id).ok_or(Error::PlanNotFound)?;
            if !plan.active || plan.operator != operator {
                return Err(Error::PlanInactive);
            }

            if storage::get_lease(&e, item.lease_id).is_some() {
                return Err(Error::LeaseAlreadyExists);
            }

            if storage::get_device_lease(&e, &item.device_id).is_some() {
                return Err(Error::DeviceAlreadyAssigned);
            }

            if let Some(pid) = item.pool_id {
                let pool = storage::get_pool(&e, pid).ok_or(Error::PoolNotFound)?;
                if pool.operator != operator {
                    return Err(Error::Unauthorized);
                }
            }

            let lease = Lease {
                lease_id: item.lease_id,
                operator: operator.clone(),
                customer: item.customer.clone(),
                plan_id: item.plan_id,
                device_id: item.device_id.clone(),
                pool_id: item.pool_id,
                status: LeaseStatus::Active,
                paid_until: current_time,
                total_paid: 0,
                emergency_paused_until: 0,
                created_at: current_time,
                last_payment_at: current_time,
            };

            storage::set_lease(&e, &lease);
            storage::set_device_lease(&e, &item.device_id, item.lease_id);
            events::emit_lease_created(&e, item.lease_id, item.customer, operator.clone(), item.device_id, item.pool_id);
            count += 1;
        }

        op.total_leases += count as u64;
        storage::set_operator(&e, &op);

        Ok(count)
    }

    pub fn calculate_access_seconds(amount: i128, daily_rate: i128) -> u64 {
        if daily_rate <= 0 || amount <= 0 {
            return 0;
        }
        let total_seconds_scaled = amount.saturating_mul(SECONDS_PER_DAY as i128);
        (total_seconds_scaled / daily_rate) as u64
    }

    pub fn pay(e: Env, payer: Address, lease_id: u64, amount: i128) -> Result<i128, Error> {
        payer.require_auth();

        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        let plan = storage::get_plan(&e, lease.plan_id).ok_or(Error::PlanNotFound)?;
        let mut op = storage::get_operator(&e, &lease.operator).ok_or(Error::OperatorNotFound)?;

        if lease.status == LeaseStatus::Owned {
            return Err(Error::LeaseAlreadyOwned);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }
        if lease.status == LeaseStatus::Suspended {
            return Err(Error::LeaseSuspended);
        }

        let remaining_to_own = plan.total_price - lease.total_paid;
        if remaining_to_own <= 0 {
            return Err(Error::LeaseAlreadyOwned);
        }

        let actual_payment = if amount > remaining_to_own {
            remaining_to_own
        } else {
            amount
        };

        if actual_payment < plan.min_payment && actual_payment < remaining_to_own {
            return Err(Error::BelowMinimumPayment);
        }

        let mut pool_opt = lease.pool_id.and_then(|pid| storage::get_pool(&e, pid));

        let (op_share, pool_share) = Self::internal_split_and_transfer(
            &e,
            &payer,
            &plan.token,
            actual_payment,
            &op,
            &mut pool_opt,
        );

        if let Some(ref p) = pool_opt {
            storage::set_pool(&e, p);
        }

        op.total_volume_collected += op_share;
        storage::set_operator(&e, &op);

        let current_time = e.ledger().timestamp();
        let seconds_added = Self::calculate_access_seconds(actual_payment, plan.daily_rate);

        let base_time = if lease.paid_until > current_time {
            lease.paid_until
        } else {
            current_time
        };

        lease.total_paid += actual_payment;
        lease.last_payment_at = current_time;

        let is_owned = lease.total_paid >= plan.total_price;
        if is_owned {
            lease.status = LeaseStatus::Owned;
            lease.paid_until = u64::MAX;
        } else {
            lease.paid_until = base_time + seconds_added;
        }

        storage::set_lease(&e, &lease);

        events::emit_payment(
            &e,
            lease_id,
            payer,
            actual_payment,
            lease.paid_until,
            lease.total_paid,
            op_share,
            pool_share,
        );

        if is_owned {
            events::emit_owned(&e, lease_id, lease.customer.clone(), lease.total_paid);
        }

        Ok(actual_payment)
    }

    pub fn grant_credit(
        e: Env,
        operator: Address,
        lease_id: u64,
        days: u32,
        reason: String,
    ) -> Result<u64, Error> {
        operator.require_auth();

        if days == 0 {
            return Err(Error::InvalidAmount);
        }

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.operator != operator {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }
        if lease.status == LeaseStatus::Owned {
            return Err(Error::LeaseAlreadyOwned);
        }

        let current_time = e.ledger().timestamp();
        let seconds_added = (days as u64) * SECONDS_PER_DAY;
        let base_time = if lease.paid_until > current_time {
            lease.paid_until
        } else {
            current_time
        };

        lease.paid_until = base_time + seconds_added;
        storage::set_lease(&e, &lease);

        events::emit_credit_granted(&e, lease_id, operator, days, lease.paid_until, reason);
        Ok(lease.paid_until)
    }

    pub fn request_plan_change(
        e: Env,
        operator: Address,
        lease_id: u64,
        new_plan_id: u32,
    ) -> Result<(), Error> {
        operator.require_auth();

        let lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.operator != operator {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Owned {
            return Err(Error::LeaseAlreadyOwned);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }
        if lease.plan_id == new_plan_id {
            return Err(Error::InvalidPlanParams);
        }

        let new_plan = storage::get_plan(&e, new_plan_id).ok_or(Error::PlanNotFound)?;
        if !new_plan.active || new_plan.operator != operator {
            return Err(Error::PlanInactive);
        }

        storage::set_pending_plan_change(&e, lease_id, new_plan_id);
        events::emit_plan_change_requested(&e, lease_id, lease.plan_id, new_plan_id);
        Ok(())
    }

    pub fn accept_plan_change(e: Env, customer: Address, lease_id: u64) -> Result<(), Error> {
        customer.require_auth();

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.customer != customer {
            return Err(Error::Unauthorized);
        }

        let new_plan_id = storage::get_pending_plan_change(&e, lease_id).ok_or(Error::NoPendingPlanChange)?;
        let new_plan = storage::get_plan(&e, new_plan_id).ok_or(Error::PlanNotFound)?;
        if !new_plan.active {
            return Err(Error::PlanInactive);
        }

        let old_plan = lease.plan_id;
        lease.plan_id = new_plan_id;
        storage::set_lease(&e, &lease);
        storage::clear_pending_plan_change(&e, lease_id);

        events::emit_plan_change_accepted(&e, lease_id, old_plan, new_plan_id);
        Ok(())
    }

    pub fn get_pending_plan_change(e: Env, lease_id: u64) -> Result<u32, Error> {
        storage::get_pending_plan_change(&e, lease_id).ok_or(Error::NoPendingPlanChange)
    }

    pub fn device_swap(
        e: Env,
        operator: Address,
        lease_id: u64,
        new_device_id: BytesN<32>,
        reason: String,
    ) -> Result<(), Error> {
        operator.require_auth();

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.operator != operator {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }

        if storage::get_device_lease(&e, &new_device_id).is_some() {
            return Err(Error::DeviceAlreadyAssigned);
        }

        let old_device = lease.device_id.clone();
        storage::remove_device_lease(&e, &old_device);
        storage::set_device_lease(&e, &new_device_id, lease_id);

        lease.device_id = new_device_id.clone();
        storage::set_lease(&e, &lease);

        events::emit_device_swapped(&e, lease_id, operator, old_device, new_device_id, reason);
        Ok(())
    }

    pub fn transfer_lease_customer(
        e: Env,
        current_customer: Address,
        new_customer: Address,
        lease_id: u64,
    ) -> Result<(), Error> {
        current_customer.require_auth();

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.customer != current_customer {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }

        let old_customer = lease.customer.clone();
        lease.customer = new_customer.clone();
        storage::set_lease(&e, &lease);

        events::emit_lease_transferred(&e, lease_id, old_customer, new_customer);
        Ok(())
    }

    pub fn emergency_pause(
        e: Env,
        operator: Address,
        lease_id: u64,
        pause_duration_seconds: u64,
        reason: String,
    ) -> Result<u64, Error> {
        operator.require_auth();

        if pause_duration_seconds == 0 {
            return Err(Error::PauseDurationInvalid);
        }

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.operator != operator {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Owned {
            return Err(Error::LeaseAlreadyOwned);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }

        let current_time = e.ledger().timestamp();
        // If lease was active/within access, shift expiration forward by pause duration
        if lease.paid_until > current_time {
            lease.paid_until += pause_duration_seconds;
        } else {
            // If already expired, grant grace relief during the crisis
            lease.paid_until = current_time + pause_duration_seconds;
        }

        lease.emergency_paused_until = current_time + pause_duration_seconds;
        storage::set_lease(&e, &lease);

        events::emit_emergency_pause(&e, lease_id, operator, lease.emergency_paused_until, reason);
        Ok(lease.emergency_paused_until)
    }

    pub fn set_suspended(
        e: Env,
        operator: Address,
        lease_id: u64,
        suspended: bool,
        reason: String,
    ) -> Result<(), Error> {
        operator.require_auth();

        let mut lease = storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)?;
        if lease.operator != operator {
            return Err(Error::Unauthorized);
        }
        if lease.status == LeaseStatus::Owned {
            return Err(Error::LeaseAlreadyOwned);
        }
        if lease.status == LeaseStatus::Repossessed {
            return Err(Error::LeaseRepossessed);
        }

        lease.status = if suspended {
            LeaseStatus::Suspended
        } else {
            LeaseStatus::Active
        };

        storage::set_lease(&e, &lease);
        events::emit_suspended(&e, lease_id, operator, suspended, reason);
        Ok(())
    }

    pub fn get_lease(e: Env, lease_id: u64) -> Result<Lease, Error> {
        storage::get_lease(&e, lease_id).ok_or(Error::LeaseNotFound)
    }

    pub fn version(_e: Env) -> u32 {
        1
    }
}
