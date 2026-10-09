use soroban_sdk::Env;

use crate::types::{DataKey, Plan, Subscription};

const DAY_IN_LEDGERS: u32 = 17_280;
const TTL_THRESHOLD: u32 = 15 * DAY_IN_LEDGERS;
const TTL_EXTEND_TO: u32 = 30 * DAY_IN_LEDGERS;

pub fn extend_instance(env: &Env) {
    env.storage()
        .instance()
        .extend_ttl(TTL_THRESHOLD, TTL_EXTEND_TO);
}

fn next_id(env: &Env, key: &DataKey) -> u64 {
    let id: u64 = env.storage().instance().get(key).unwrap_or(0u64) + 1;
    env.storage().instance().set(key, &id);
    id
}

fn count(env: &Env, key: &DataKey) -> u64 {
    env.storage().instance().get(key).unwrap_or(0u64)
}

pub fn next_plan_id(env: &Env) -> u64 {
    next_id(env, &DataKey::PlanCount)
}

pub fn next_sub_id(env: &Env) -> u64 {
    next_id(env, &DataKey::SubCount)
}

pub fn plan_count(env: &Env) -> u64 {
    count(env, &DataKey::PlanCount)
}

pub fn sub_count(env: &Env) -> u64 {
    count(env, &DataKey::SubCount)
}

pub fn get_plan(env: &Env, id: u64) -> Option<Plan> {
    let key = DataKey::Plan(id);
    let plan: Option<Plan> = env.storage().persistent().get(&key);
    if plan.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    plan
}

pub fn set_plan(env: &Env, id: u64, plan: &Plan) {
    let key = DataKey::Plan(id);
    env.storage().persistent().set(&key, plan);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}

pub fn get_sub(env: &Env, id: u64) -> Option<Subscription> {
    let key = DataKey::Sub(id);
    let sub: Option<Subscription> = env.storage().persistent().get(&key);
    if sub.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
    }
    sub
}

pub fn set_sub(env: &Env, id: u64, sub: &Subscription) {
    let key = DataKey::Sub(id);
    env.storage().persistent().set(&key, sub);
    env.storage()
        .persistent()
        .extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}
