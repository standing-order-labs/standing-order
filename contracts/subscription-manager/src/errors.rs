use soroban_sdk::contracterror;

/// Errors returned by the `subscription-manager` contract.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    /// Plan amount must be strictly positive.
    InvalidAmount = 1,
    /// Plan period is shorter than `MIN_PERIOD_SECS`.
    InvalidPeriod = 2,
    /// No plan exists with the given id.
    PlanNotFound = 3,
    /// The plan was deactivated by its merchant.
    PlanInactive = 4,
    /// No subscription exists with the given id.
    SubscriptionNotFound = 5,
    /// The subscription has been cancelled.
    SubscriptionNotActive = 6,
    /// The next charge is not due yet.
    NotDue = 7,
    /// The token transfer failed (usually insufficient balance or allowance).
    PaymentFailed = 8,
    /// The plan is already deactivated.
    PlanAlreadyInactive = 9,
}
