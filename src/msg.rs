use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use crate::state::{Deposit, Withdraw, Reward};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
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
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    DepositUst { entry_address: String, amount: u64 },
    WithdrawUst { entry_address: String, amount: u64 },
    ClaimReward { entry_address: String },
    SellReward { entry_address: String },
    UpdateStateAndEntries {},
    CashoutYield { },
    SetProtocolWallet {},
    SetTreasuryWallet {},
    SetRewardContract {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetEntry { entry_address: String },
    GetState {},
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EntryResponse {
    pub claimable_reward: u64,
    pub ust_deposited: u64,
    pub averaged_reward_rate: f64,
    pub ust_deposit_log: Vec<Deposit>,
    pub ust_withdraw_log: Vec<Withdraw>,
    pub dynamic_reward_log: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateResponse {
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
