use cosmwasm_std::{DepsMut, Response, MessageInfo, StdResult, Order, Uint128, Env, SubMsg, BankMsg, coins, Coin, Decimal256, Uint256, CanonicalAddr, CosmosMsg, WasmMsg, to_binary};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::handler::helper as ExecuteHelper;
use crate::{ContractError, state::{ENTRIES, STATE, Entry}};

// FIX this
use cw20::Cw20ExecuteMsg;
use terra_cosmwasm::TerraQuerier;

pub fn try_deposit(deps: DepsMut, info: MessageInfo, env: Env, entry_address: String, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let time = env.block.time.seconds();
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => ExecuteHelper::some_deposit_helper(entry, amount, time),
            None => ExecuteHelper::none_deposit_helper(amount, time),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        // transfer funds from users wallet to protocol wallet
        let mut response: Response = Default::default();
        let coin_amount = coins(amount.u128(), "uust");
        response.messages = vec![SubMsg::new(BankMsg::Send {
            to_address: env.contract.address.to_string(),
            amount: coin_amount,
        })];
        // swap ust for aust
        let deposit = deposit_stable_msg(deps, &state.anc_market, "uust", amount);
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_deposit"))
}

pub fn try_withdraw(deps: DepsMut, info: MessageInfo, env: Env, entry_address: String, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let time = env.block.time.seconds();
    let update_entry = |entry: Option<Entry>| -> Result<Entry, ContractError> {
        match entry {
            Some(entry) => ExecuteHelper::some_withdraw_helper(entry, time, amount),
            None => Err(ContractError::CannotWithdrawWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        // swap from aust to ust
        let withdraw = redeem_stable_msg(deps, &state.anc_market, &state.aust_contract, amount);
        // transfer funds from protocol to users wallet
        let mut response: Response = Default::default();
        let coin_amount = coins(amount.u128(), "uust");
        response.messages = vec![SubMsg::new(BankMsg::Send {
            to_address: entry_address.to_string(),
            amount: coin_amount,
        })];
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
        // TODO: remember to update state variables in all these functions
        // STATE.update(deps.storage, action)
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_withdraw"))
}

pub fn try_claim(deps: DepsMut, info: MessageInfo, entry_address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let update_entry = |entry: Option<Entry>| -> Result<Entry, ContractError> {
        match entry {
            Some(entry) => ExecuteHelper::some_claim_helper(info, entry),
            None => Err(ContractError::CannotClaimWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_claim"))
}

pub fn try_sell(deps: DepsMut, info: MessageInfo, env: Env, entry_address: String, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    
    if info.sender == entry_address || info.sender == state.owner {
        // TODO: make sure they have amount of MIN in wallet to sell, 
        // transfer UST from protocol to users wallet, 
        let mut response: Response = Default::default();
        let coin_amount = coins(amount.u128(), "uust");
        response.messages = vec![SubMsg::new(BankMsg::Send {
            to_address: entry_address,
            amount: coin_amount,
        })];
        // transfer MIN from users wallet to protocol wallet,
        let mut response: Response = Default::default();
        let coin_amount = coins(amount.u128(), "umin");
        response.messages = vec![SubMsg::new(BankMsg::Send {
            to_address: env.contract.address.to_string(),
            amount: coin_amount,
        })];
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_sell"))
}

pub fn try_update_entries(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let entries: StdResult<Vec<_>> = ENTRIES.range(deps.storage, None, None, Order::Ascending).collect();

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO: Update claimable reward, update average reward rate, update dynamic reward log
        for vec in entries {
            for tuple in vec {
                let entry = tuple.1;

            }
        }
        
    }
    Ok(Response::new().add_attribute("method", "try_update_state_and_entries"))
}

pub fn try_cashout_yield(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO: subtract balance in protocol wallet from state variable total ust deposited
        // transfer that amount to treasury wallet
    }
    Ok(Response::new().add_attribute("method", "try_cashout_yield"))
}

pub fn try_set_treasury_wallet(deps: DepsMut, info: MessageInfo, address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;   
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        let valid_address = deps.api.addr_validate(&address)?;
        state.treasury_wallet = valid_address;
        STATE.save(deps.storage, &state)?;
    }
    Ok(Response::new().add_attribute("method", "try_set_treasury_wallet"))
}

pub fn try_set_reward_contract(deps: DepsMut, info: MessageInfo, address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        let valid_address = deps.api.addr_validate(&address)?;
        state.reward_contract = valid_address;
        STATE.save(deps.storage, &state)?;
    }
    Ok(Response::new().add_attribute("method", "try_set_reward_contract"))
}

pub fn try_set_tier_data(deps: DepsMut, info: MessageInfo, data: (u8, f64, u64)) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        if data.0 == 0 {
            state.tier0rate = data.1;
            state.tier0time = data.2;
        } else if data.0 == 1 {
            state.tier1rate = data.1;
            state.tier1time = data.2;
        } else if data.0 == 2 {
            state.tier2rate = data.1;
            state.tier2time = data.2;
        } else if data.0 == 3 {
            state.tier3rate = data.1;
            state.tier3time = data.2;
        } else {
            return Err(ContractError::Unauthorized {});
        }
        STATE.save(deps.storage, &state)?;
    }
    Ok(Response::new().add_attribute("method", "try_set_tier_data"))
}

pub fn try_set_anc_market(deps: DepsMut, info: MessageInfo, address: CanonicalAddr) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        state.anc_market = address;
        STATE.save(deps.storage, &state)?;
    }
    Ok(Response::new().add_attribute("method", "try_set_tier_data"))
}

// helpers, move to helper file

// Figure out these errors, see if they even are errors because pylon uses them still or so it seems
pub fn compute_tax(deps: DepsMut, coin: &Coin) -> StdResult<Uint256> {
    let terra_querier = TerraQuerier::new(&deps.querier);
    let tax_rate = Decimal256::from((terra_querier.query_tax_rate()?).rate);
    let tax_cap = Uint256::from((terra_querier.query_tax_cap(coin.denom.to_string())?).cap);
    let amount = Uint256::from(coin.amount);
    Ok(std::cmp::min(
        amount * Decimal256::one() - amount / (Decimal256::one() + tax_rate),
        tax_cap,
    ))
}

pub fn deduct_tax(deps: DepsMut, coin: Coin) -> StdResult<Coin> {
    let tax_amount = compute_tax(deps, &coin)?;
    Ok(Coin {
        denom: coin.denom,
        amount: (Uint256::from(coin.amount) - tax_amount).into(),
    })
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    DepositStable {},
}

pub fn deposit_stable_msg(
    deps: DepsMut,
    market: &CanonicalAddr,
    denom: &str,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw20HookMsg {
    /// Return stable coins to a user
    /// according to exchange rate
    RedeemStable {},
}

pub fn redeem_stable_msg(
    deps: DepsMut,
    market: &CanonicalAddr,
    token: &CanonicalAddr,
    amount: Uint128,
) -> StdResult<Vec<CosmosMsg>> {
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