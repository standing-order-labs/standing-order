#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

use crate::{
    Error, SubscriptionManager, SubscriptionManagerClient, SubscriptionStatus, MIN_PERIOD_SECS,
};

const START: u64 = 1_700_000_000;
const PERIOD: u64 = 30 * 24 * 3_600;
const PRICE: i128 = 10_000_000;
const APPROVE_UNTIL: u32 = 100_000;

struct Ctx<'a> {
    env: &'a Env,
    client: SubscriptionManagerClient<'a>,
    token: TokenClient<'a>,
    token_admin: StellarAssetClient<'a>,
    contract_id: Address,
    merchant: Address,
    subscriber: Address,
}

fn setup(env: &Env) -> Ctx<'_> {
    env.mock_all_auths();
    env.ledger().set_timestamp(START);

    let admin = Address::generate(env);
    let token_addr = env.register_stellar_asset_contract_v2(admin).address();
    let contract_id = env.register(SubscriptionManager, ());

    let token = TokenClient::new(env, &token_addr);
    let token_admin = StellarAssetClient::new(env, &token_addr);
    let client = SubscriptionManagerClient::new(env, &contract_id);

    let merchant = Address::generate(env);
    let subscriber = Address::generate(env);
    token_admin.mint(&subscriber, &(PRICE * 10));
    token.approve(&subscriber, &contract_id, &(PRICE * 10), &APPROVE_UNTIL);

    Ctx {
        env,
        client,
        token,
        token_admin,
        contract_id,
        merchant,
        subscriber,
    }
}

fn advance(env: &Env, secs: u64) {
    env.ledger().with_mut(|l| l.timestamp += secs);
}

fn new_plan(c: &Ctx) -> u64 {
    c.client
        .create_plan(&c.merchant, &c.token.address, &PRICE, &PERIOD)
}

// ------------------------------------------------------------------ plans

#[test]
fn create_plan_stores_plan() {
    let env = Env::default();
    let c = setup(&env);
    let id = new_plan(&c);

    assert_eq!(id, 1);
    assert_eq!(c.client.plan_count(), 1);
    let plan = c.client.get_plan(&id);
    assert_eq!(plan.merchant, c.merchant);
    assert_eq!(plan.token, c.token.address);
    assert_eq!(plan.amount, PRICE);
    assert_eq!(plan.period, PERIOD);
    assert!(plan.active);
}

#[test]
fn plan_ids_increment() {
    let env = Env::default();
    let c = setup(&env);
    assert_eq!(new_plan(&c), 1);
    assert_eq!(new_plan(&c), 2);
    assert_eq!(c.client.plan_count(), 2);
}

#[test]
fn create_plan_rejects_non_positive_amount() {
    let env = Env::default();
    let c = setup(&env);
    let res = c
        .client
        .try_create_plan(&c.merchant, &c.token.address, &0, &PERIOD);
    assert_eq!(res, Err(Ok(Error::InvalidAmount)));
}

#[test]
fn create_plan_rejects_short_period() {
    let env = Env::default();
    let c = setup(&env);
    let res = c.client.try_create_plan(
        &c.merchant,
        &c.token.address,
        &PRICE,
        &(MIN_PERIOD_SECS - 1),
    );
    assert_eq!(res, Err(Ok(Error::InvalidPeriod)));
}

#[test]
fn get_plan_unknown_id_fails() {
    let env = Env::default();
    let c = setup(&env);
    assert_eq!(c.client.try_get_plan(&42), Err(Ok(Error::PlanNotFound)));
}

#[test]
fn deactivate_plan_marks_inactive_and_blocks_new_subscribers() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);

    c.client.deactivate_plan(&plan_id);
    assert!(!c.client.get_plan(&plan_id).active);
    assert_eq!(
        c.client.try_subscribe(&c.subscriber, &plan_id),
        Err(Ok(Error::PlanInactive))
    );
    assert_eq!(
        c.client.try_deactivate_plan(&plan_id),
        Err(Ok(Error::PlanAlreadyInactive))
    );
}

// ------------------------------------------------------------- subscribing

#[test]
fn subscribe_pays_first_period_immediately() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);

    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    assert_eq!(sub_id, 1);
    assert_eq!(c.token.balance(&c.merchant), PRICE);
    assert_eq!(c.token.balance(&c.subscriber), PRICE * 9);
    // The contract never custodies funds.
    assert_eq!(c.token.balance(&c.contract_id), 0);

    let sub = c.client.get_subscription(&sub_id);
    assert_eq!(sub.plan_id, plan_id);
    assert_eq!(sub.subscriber, c.subscriber);
    assert_eq!(sub.charges, 1);
    assert_eq!(sub.next_charge, START + PERIOD);
    assert_eq!(sub.status, SubscriptionStatus::Active);
}

#[test]
fn subscribe_unknown_plan_fails() {
    let env = Env::default();
    let c = setup(&env);
    assert_eq!(
        c.client.try_subscribe(&c.subscriber, &7),
        Err(Ok(Error::PlanNotFound))
    );
}

// ---------------------------------------------------------------- charging

#[test]
fn charge_before_due_fails() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    advance(c.env, PERIOD - 1);
    assert!(!c.client.is_due(&sub_id));
    assert_eq!(c.client.try_charge(&sub_id), Err(Ok(Error::NotDue)));
}

#[test]
fn charge_after_period_pays_merchant_and_advances_schedule() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    advance(c.env, PERIOD);
    assert!(c.client.is_due(&sub_id));
    c.client.charge(&sub_id);

    assert_eq!(c.token.balance(&c.merchant), PRICE * 2);
    assert_eq!(c.token.balance(&c.subscriber), PRICE * 8);

    let sub = c.client.get_subscription(&sub_id);
    assert_eq!(sub.charges, 2);
    assert_eq!(sub.next_charge, START + 2 * PERIOD);
    assert!(!c.client.is_due(&sub_id));
}

#[test]
fn charge_cannot_be_repeated_in_the_same_period() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    advance(c.env, PERIOD);
    c.client.charge(&sub_id);
    assert_eq!(c.client.try_charge(&sub_id), Err(Ok(Error::NotDue)));
}

#[test]
fn charge_skips_missed_periods_instead_of_back_billing() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    advance(c.env, 3 * PERIOD);
    c.client.charge(&sub_id);

    let sub = c.client.get_subscription(&sub_id);
    assert_eq!(sub.charges, 2);
    assert_eq!(sub.next_charge, START + 3 * PERIOD + PERIOD);
    assert_eq!(c.client.try_charge(&sub_id), Err(Ok(Error::NotDue)));
}

#[test]
fn charge_fails_without_allowance_and_leaves_state_untouched() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);

    // A second subscriber who funds the account but never approves the contract.
    let lazy = Address::generate(c.env);
    c.token_admin.mint(&lazy, &(PRICE * 2));
    let sub_id = c.client.subscribe(&lazy, &plan_id);

    advance(c.env, PERIOD);
    assert_eq!(c.client.try_charge(&sub_id), Err(Ok(Error::PaymentFailed)));

    let sub = c.client.get_subscription(&sub_id);
    assert_eq!(sub.charges, 1);
    assert_eq!(sub.next_charge, START + PERIOD);
    assert_eq!(c.token.balance(&lazy), PRICE);
}

#[test]
fn charge_unknown_subscription_fails() {
    let env = Env::default();
    let c = setup(&env);
    assert_eq!(
        c.client.try_charge(&99),
        Err(Ok(Error::SubscriptionNotFound))
    );
    assert!(!c.client.is_due(&99));
}

#[test]
fn charge_blocked_after_plan_deactivated() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    c.client.deactivate_plan(&plan_id);
    advance(c.env, PERIOD);

    assert!(!c.client.is_due(&sub_id));
    assert_eq!(c.client.try_charge(&sub_id), Err(Ok(Error::PlanInactive)));
}

// -------------------------------------------------------------- cancelling

#[test]
fn cancel_stops_future_charges() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    c.client.cancel(&sub_id);
    assert_eq!(
        c.client.get_subscription(&sub_id).status,
        SubscriptionStatus::Cancelled
    );

    advance(c.env, PERIOD);
    assert!(!c.client.is_due(&sub_id));
    assert_eq!(
        c.client.try_charge(&sub_id),
        Err(Ok(Error::SubscriptionNotActive))
    );
}

#[test]
fn cancel_twice_fails() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    c.client.cancel(&sub_id);
    assert_eq!(
        c.client.try_cancel(&sub_id),
        Err(Ok(Error::SubscriptionNotActive))
    );
}

#[test]
fn cancel_requires_subscriber_authorization() {
    let env = Env::default();
    let c = setup(&env);
    let plan_id = new_plan(&c);
    let sub_id = c.client.subscribe(&c.subscriber, &plan_id);

    c.client.cancel(&sub_id);

    let auths = env.auths();
    assert_eq!(auths.len(), 1);
    assert_eq!(auths[0].0, c.subscriber);
}
