use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub count: i32,

    pub reward_contract: Addr,
    pub total_ust_deposited: i32,
    pub total_rewards_claimed: i32,
    pub reward_teirs: vec<RewardTeir>,
    pub fixed_price: i32,
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RewardTeir {
    pub time: i32,
    pub rate: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub owner: Addr,
    pub claimable_reward: i32,
    pub ust_balance: i32,
    pub reward_rate: i32,
    pub ust_deposit_log: vec<Deposit>,
    pub ust_withdraw_log: vec<Withdraw>,
    pub dynamic_reward_log: vec<Reward>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub amount: i32,
    pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraw {
    pub amount: i32,
    pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
    pub amount: i32,
    pub time: Timestamp,
    pub reward_rate: i32,
}

pub const STATE: Item<State> = Item::new("state");
pub const ENTRIES: Map<&Addr, Entry> = Map::new("entries");
