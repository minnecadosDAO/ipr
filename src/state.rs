use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub treasury_wallet: Addr,
    pub reward_contract: Addr,
    pub ust_deposited: Uint128,
    pub sellback_price: u64,
    pub anc_market: Addr,
    pub aust_contract: Addr,
    pub tier0rate: u64,
    pub tier0time: u64,
    pub tier1rate: u64,
    pub tier1time: u64,
    pub tier2rate: u64,
    pub tier2time: u64,
    pub tier3rate: u64,
    pub tier3time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub claimable_reward: Uint128,
    pub ust_deposited: Uint128,
    pub averaged_reward_rate: u64,
    pub ust_deposit_log: Vec<Deposit>,
    pub ust_withdraw_log: Vec<Withdraw>,
    pub dynamic_reward_log: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Deposit {
    pub amount: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Withdraw {
    pub amount: Uint128,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Reward {
    pub amount: Uint128,
    pub time: u64,
    pub reward_tier: u8,
}

pub const STATE: Item<State> = Item::new("state");
pub const ENTRIES: Map<&Addr, Entry> = Map::new("entries");
