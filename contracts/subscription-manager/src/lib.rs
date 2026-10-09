//! # Standing Order - `subscription-manager`
//!
//! Pull-based recurring payments for any SEP-41 token on Soroban.
//!
//! * A **merchant** publishes a [`Plan`] (token, amount, period).
//! * A **subscriber** calls [`SubscriptionManager::subscribe`], pays the first period
//!   immediately, and grants this contract a token allowance for later periods.
//! * **Anyone** (a keeper bot, the merchant, the subscriber) may call
//!   [`SubscriptionManager::charge`] once a period has elapsed. Funds always move
//!   directly from subscriber to merchant; the contract never holds balances.
//!
//! See `docs/ARCHITECTURE.md` for the full design.
#![no_std]

mod errors;
mod events;
mod storage;
mod types;

#[cfg(test)]
mod test;

pub use errors::Error;
pub use types::{Plan, Subscription, SubscriptionStatus};

use soroban_sdk::{contract, contractimpl, token, Address, Env};

/// Shortest allowed billing period (one hour). Guards against accidental
/// "charge every second" plans.
pub const MIN_PERIOD_SECS: u64 = 3_600;

#[contract]
pub struct SubscriptionManager;

#[contractimpl]
impl SubscriptionManager {
    // ---------------------------------------------------------------- merchants

    /// Create a plan. Requires the merchant's authorization. Returns the plan id.
    pub fn create_plan(
        env: Env,
        merchant: Address,
        token: Address,
        amount: i128,
        period: u64,
    ) -> Result<u64, Error> {
        merchant.require_auth();
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        if period < MIN_PERIOD_SECS {
            return Err(Error::InvalidPeriod);
        }

        storage::extend_instance(&env);
        let plan_id = storage::next_plan_id(&env);
        let plan = Plan {
            merchant: merchant.clone(),
            token: token.clone(),
            amount,
            period,
            active: true,
        };
        storage::set_plan(&env, plan_id, &plan);

        events::PlanCreated {
            plan_id,
            merchant,
            token,
            amount,
            period,
        }
        .publish(&env);
        Ok(plan_id)
    }

    /// Stop a plan. No new subscriptions are accepted and existing ones can no
    /// longer be charged. Requires the merchant's authorization.
    pub fn deactivate_plan(env: Env, plan_id: u64) -> Result<(), Error> {
        let mut plan = storage::get_plan(&env, plan_id).ok_or(Error::PlanNotFound)?;
        plan.merchant.require_auth();
        if !plan.active {
            return Err(Error::PlanAlreadyInactive);
        }

        storage::extend_instance(&env);
        plan.active = false;
        storage::set_plan(&env, plan_id, &plan);
        events::PlanDeactivated { plan_id }.publish(&env);
        Ok(())
    }

    // -------------------------------------------------------------- subscribers

    /// Subscribe to a plan. The first period is paid immediately and directly to
    /// the merchant. To allow later charges the subscriber must also call the
    /// token's `approve(subscriber, <this contract>, amount, live_until_ledger)`.
    pub fn subscribe(env: Env, subscriber: Address, plan_id: u64) -> Result<u64, Error> {
        subscriber.require_auth();
        let plan = storage::get_plan(&env, plan_id).ok_or(Error::PlanNotFound)?;
        if !plan.active {
            return Err(Error::PlanInactive);
        }

        token::TokenClient::new(&env, &plan.token).transfer(
            &subscriber,
            &plan.merchant,
            &plan.amount,
        );

        storage::extend_instance(&env);
        let sub_id = storage::next_sub_id(&env);
        let sub = Subscription {
            plan_id,
            subscriber: subscriber.clone(),
            next_charge: env.ledger().timestamp().saturating_add(plan.period),
            charges: 1,
            status: SubscriptionStatus::Active,
        };
        storage::set_sub(&env, sub_id, &sub);

        events::Subscribed {
            sub_id,
            plan_id,
            subscriber,
        }
        .publish(&env);
        Ok(sub_id)
    }

    /// Cancel a subscription. Requires the subscriber's authorization.
    /// Subscribers should also revoke their token allowance.
    pub fn cancel(env: Env, sub_id: u64) -> Result<(), Error> {
        let mut sub = storage::get_sub(&env, sub_id).ok_or(Error::SubscriptionNotFound)?;
        sub.subscriber.require_auth();
        if sub.status != SubscriptionStatus::Active {
            return Err(Error::SubscriptionNotActive);
        }

        storage::extend_instance(&env);
        sub.status = SubscriptionStatus::Cancelled;
        storage::set_sub(&env, sub_id, &sub);
        events::Cancelled { sub_id }.publish(&env);
        Ok(())
    }

    // ------------------------------------------------------------------ keepers

    /// Pull the next payment for a subscription. Permissionless: anyone may call
    /// it once the subscription is due. Missed periods are skipped, not back-billed.
    pub fn charge(env: Env, sub_id: u64) -> Result<(), Error> {
        let mut sub = storage::get_sub(&env, sub_id).ok_or(Error::SubscriptionNotFound)?;
        if sub.status != SubscriptionStatus::Active {
            return Err(Error::SubscriptionNotActive);
        }
        let plan = storage::get_plan(&env, sub.plan_id).ok_or(Error::PlanNotFound)?;
        if !plan.active {
            return Err(Error::PlanInactive);
        }

        let now = env.ledger().timestamp();
        if now < sub.next_charge {
            return Err(Error::NotDue);
        }

        let res = token::TokenClient::new(&env, &plan.token).try_transfer_from(
            &env.current_contract_address(),
            &sub.subscriber,
            &plan.merchant,
            &plan.amount,
        );
        if !matches!(res, Ok(Ok(()))) {
            return Err(Error::PaymentFailed);
        }

        storage::extend_instance(&env);
        let mut next = sub.next_charge.saturating_add(plan.period);
        if next <= now {
            // Keeper was late by more than one period: skip the missed ones.
            next = now.saturating_add(plan.period);
        }
        sub.next_charge = next;
        sub.charges = sub.charges.saturating_add(1);
        storage::set_sub(&env, sub_id, &sub);

        events::Charged {
            sub_id,
            amount: plan.amount,
            charges: sub.charges,
            next_charge: next,
        }
        .publish(&env);
        Ok(())
    }

    // -------------------------------------------------------------------- views

    pub fn get_plan(env: Env, plan_id: u64) -> Result<Plan, Error> {
        storage::get_plan(&env, plan_id).ok_or(Error::PlanNotFound)
    }

    pub fn get_subscription(env: Env, sub_id: u64) -> Result<Subscription, Error> {
        storage::get_sub(&env, sub_id).ok_or(Error::SubscriptionNotFound)
    }

    pub fn plan_count(env: Env) -> u64 {
        storage::plan_count(&env)
    }

    pub fn subscription_count(env: Env) -> u64 {
        storage::sub_count(&env)
    }

    /// `true` if `charge(sub_id)` would pass its time and status checks right
    /// now. It does not check the subscriber's balance or allowance.
    pub fn is_due(env: Env, sub_id: u64) -> bool {
        let Some(sub) = storage::get_sub(&env, sub_id) else {
            return false;
        };
        if sub.status != SubscriptionStatus::Active {
            return false;
        }
        match storage::get_plan(&env, sub.plan_id) {
            Some(plan) => plan.active && env.ledger().timestamp() >= sub.next_charge,
            None => false,
        }
    }

    /// Contract version, bumped on every release.
    pub fn version(_env: Env) -> u32 {
        1
    }
}
