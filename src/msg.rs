use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint128, CanonicalAddr};
use crate::state::{Deposit, Withdraw, Reward};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub protocol_wallet: Addr,
    pub treasury_wallet: Addr,
    pub reward_contract: Addr,
    pub ust_deposited: Uint128,
    pub sellback_price: u64,
    pub anc_market: CanonicalAddr,
    pub aust_contract: CanonicalAddr,
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
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    DepositUst { entry_address: String, amount: Uint128 },
    WithdrawUst { entry_address: String, amount: Uint128 },
    ClaimReward { entry_address: String },
    SellReward { entry_address: String, amount: Uint128 },
    UpdateEntries {},
    CashoutYield {},
    SetTreasuryWallet { address: String },
    SetRewardContract { address: String },
    SetTierData { data: (u8, f64, u64) },
    SetAncMarket { address: CanonicalAddr },
    Send { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetEntry { entry_address: String },
    GetState {},
    Balance {},

}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub claimable_reward: Uint128,
    pub ust_deposited: Uint128,
    pub averaged_reward_rate: f64,
    pub ust_deposit_log: Vec<Deposit>,
    pub ust_withdraw_log: Vec<Withdraw>,
    pub dynamic_reward_log: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
    pub owner: Addr,
    pub treasury_wallet: Addr,
    pub reward_contract: Addr,
    pub ust_deposited: Uint128,
    pub sellback_price: u64,
    pub anc_market: CanonicalAddr,
    pub aust_contract: CanonicalAddr,
    pub tier0rate: f64,
    pub tier0time: u64,
    pub tier1rate: f64,
    pub tier1time: u64,
    pub tier2rate: f64,
    pub tier2time: u64,
    pub tier3rate: f64,
    pub tier3time: u64,
}
