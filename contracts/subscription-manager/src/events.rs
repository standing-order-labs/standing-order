use soroban_sdk::{contractevent, Address};

#[contractevent]
pub struct PlanCreated {
    #[topic]
    pub plan_id: u64,
    pub merchant: Address,
    pub token: Address,
    pub amount: i128,
    pub period: u64,
}

#[contractevent]
pub struct PlanDeactivated {
    #[topic]
    pub plan_id: u64,
}

#[contractevent]
pub struct Subscribed {
    #[topic]
    pub sub_id: u64,
    #[topic]
    pub plan_id: u64,
    pub subscriber: Address,
}

#[contractevent]
pub struct Charged {
    #[topic]
    pub sub_id: u64,
    pub amount: i128,
    pub charges: u32,
    pub next_charge: u64,
}

#[contractevent]
pub struct Cancelled {
    #[topic]
    pub sub_id: u64,
}
