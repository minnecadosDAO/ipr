#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
//RewardTiersResponse
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::state::{State, STATE};

use crate::handler::execute as ExecuteHandler;
use crate::handler::query as QueryHandler;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ipr";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        owner: info.sender.clone(),
        treasury_wallet: info.sender.clone(),
        reward_contract: info.sender.clone(),
        ust_deposited: msg.ust_deposited,
        sellback_price: msg.sellback_price,
        anc_market: msg.anc_market,
        aust_contract: msg.aust_contract,
        tier0rate: msg.tier0rate,
        tier0time: msg.tier0time,
        tier1rate: msg.tier1rate,
        tier1time: msg.tier1time,
        tier2rate: msg.tier2rate,
        tier2time: msg.tier2time,
        tier3rate: msg.tier3rate,
        tier3time: msg.tier3time,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", state.owner)
        .add_attribute("treasury_wallet", state.treasury_wallet)
        .add_attribute("reward_contract", state.reward_contract)
        .add_attribute("ust_deposited", state.ust_deposited.to_string())
        .add_attribute("sellback_price", state.sellback_price.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::DepositUst { entry_address, amount } => ExecuteHandler::try_deposit(deps, info, _env, entry_address, amount),
        ExecuteMsg::WithdrawUst { entry_address, amount } => ExecuteHandler::try_withdraw(deps, info, _env, entry_address, amount),
        ExecuteMsg::ClaimReward { entry_address } => ExecuteHandler::try_claim(deps, info, entry_address),
        ExecuteMsg::SellReward { entry_address, amount } => ExecuteHandler::try_sell(deps, info, _env, entry_address, amount),  
        ExecuteMsg::UpdateEntries {} => ExecuteHandler::try_update_entries(deps, info),  
        ExecuteMsg::CashoutYield {} => ExecuteHandler::try_cashout_yield(deps, info),
        ExecuteMsg::SetTreasuryWallet { address } => ExecuteHandler::try_set_treasury_wallet(deps, info, address),
        ExecuteMsg::SetRewardContract {address } => ExecuteHandler::try_set_reward_contract(deps, info, address),
        ExecuteMsg::SetTierData { data } => ExecuteHandler::try_set_tier_data(deps, info, data),
        ExecuteMsg::SetAncMarket { address } => ExecuteHandler::try_set_anc_market(deps, info, address),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetEntry { entry_address } => to_binary(&QueryHandler::query_entry(deps, entry_address)?),
        QueryMsg::GetState {} => to_binary(&QueryHandler::query_state(deps)?),
    }
}