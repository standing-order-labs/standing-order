use soroban_sdk::{contracttype, Address};

/// A recurring-payment offer created by a merchant.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    /// Account that receives every payment.
    pub merchant: Address,
    /// SEP-41 token used for payment.
    pub token: Address,
    /// Amount charged each period, in the token's smallest unit.
    pub amount: i128,
    /// Seconds between charges.
    pub period: u64,
    /// `false` once the merchant deactivates the plan.
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Cancelled,
}

/// A subscriber's enrolment in a plan.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Subscription {
    pub plan_id: u64,
    pub subscriber: Address,
    /// Ledger timestamp (seconds) from which the next charge may be pulled.
    pub next_charge: u64,
    /// Number of successful payments so far (including the first).
    pub charges: u32,
    pub status: SubscriptionStatus,
}

#[contracttype]
pub enum DataKey {
    PlanCount,
    SubCount,
    Plan(u64),
    Sub(u64),
}
