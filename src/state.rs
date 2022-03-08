use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::Timestamp;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub protocol_wallet: Addr,
    pub treasury_wallet: Addr,
    pub reward_contract: Addr,
    pub ust_deposited: u64,
    pub sellback_price: u64,
    pub tier0rate: f64,
    pub tier0time: u64,
    pub tier1rate: f64,
    pub tier1time: u64,
    pub tier2rate: f64,
    pub tier2time: u64,
    pub tier3rate: f64,
    pub tier3time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub claimable_reward: u64,
    pub ust_deposited: u64,
    pub averaged_reward_rate: f64,
    pub ust_deposit_log: Vec<Deposit>,
    pub ust_withdraw_log: Vec<Withdraw>,
    pub dynamic_reward_log: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub amount: u64,
    pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraw {
    pub amount: u64,
    pub time: Timestamp,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
    pub amount: u64,
    pub time: Timestamp,
    pub reward_tier: u8,
}

pub const STATE: Item<State> = Item::new("state");
pub const ENTRIES: Map<&Addr, Entry> = Map::new("entries");
