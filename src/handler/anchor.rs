use cosmwasm_bignumber::{Decimal256, Uint256};
use cosmwasm_std::*;
use cw20::Cw20ExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use terra_cosmwasm::TerraQuerier;

pub fn compute_tax(deps: Deps, coin: &Coin) -> StdResult<Uint256> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate = Decimal256::from((terra_querier.query_tax_rate()?).rate);
    let tax_cap = Uint256::from((terra_querier.query_tax_cap(coin.denom.to_string())?).cap);
    let amount = Uint256::from(coin.amount);
    Ok(std::cmp::min(
        amount * Decimal256::one() - amount / (Decimal256::one() + tax_rate),
        tax_cap,
    ))
}

pub fn deduct_tax(deps: Deps, coin: Coin) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: (Uint256::from(coin.amount) - tax_amount).into(),
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    EpochState {
        block_height: Option<u64>,
        distributed_interest: Option<Uint256>,
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner_addr: String,
    pub aterra_contract: String,
    pub interest_model: String,
    pub distribution_model: String,
    pub overseer_contract: String,
    pub collector_contract: String,
    pub distributor_contract: String,
    pub stable_denom: String,
    pub max_borrow_factor: Decimal256,
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EpochStateResponse {
    pub exchange_rate: Decimal256,
    pub aterra_supply: Uint256,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    DepositStable {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Return stable coins to a user
    /// according to exchange rate
    RedeemStable {},
}

pub fn deposit_stable_msg(deps: Deps, market: &CanonicalAddr, denom: &str, amount: Uint128,) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(market).unwrap().to_string(),
        msg: to_binary(&HandleMsg::DepositStable {})?,
        funds: vec![deduct_tax(
            deps,
            Coin {
                denom: denom.to_string(),
                amount,
            },
        )?],
    })])
}

pub fn redeem_stable_msg(deps: Deps, market: &CanonicalAddr, token: &CanonicalAddr, amount: Uint128,) -> StdResult<Vec<CosmosMsg>> {
    Ok(vec![CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: deps.api.addr_humanize(token).unwrap().to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: deps.api.addr_humanize(market).unwrap().to_string(),
            amount,
            msg: to_binary(&Cw20HookMsg::RedeemStable {}).unwrap(),
        })?,
        funds: vec![],
    })])
}