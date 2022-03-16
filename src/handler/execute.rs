use cosmwasm_std::{DepsMut, Response, MessageInfo, StdResult, Uint128, Env, SubMsg, BankMsg, coins, CanonicalAddr};
use crate::handler::helper as ExecuteHelper;
use crate::{ContractError, state::{ENTRIES, STATE, Entry}};

pub fn try_deposit(deps: DepsMut, info: MessageInfo, env: Env, entry_address: String, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let time = env.block.time.seconds();
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => ExecuteHelper::some_deposit_helper(state, deps, env, entry, amount, time),
            None => ExecuteHelper::none_deposit_helper(state, deps, env, amount, time),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
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
            Some(entry) => ExecuteHelper::some_withdraw_helper(deps, info, entry, time, amount),
            None => Err(ContractError::CannotWithdrawWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
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
    //let valid_address = deps.api.addr_validate(&entry_address)?;
    
    if info.sender == entry_address || info.sender == state.owner {
        // TODO: make sure they have amount of MIN in wallet to sell, 
        // Update here
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
    // let entries: StdResult<Vec<_>> = ENTRIES.range(deps.storage, None, None, Order::Ascending).collect();

    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO: Update claimable reward, update average reward rate, update dynamic reward log

        // loop through the dynamic reward log in each entry
        // check each entry against the time to determine if teir moves up
        // calculate averaged reward 
        // calculate how much MIN should be added to claimable reward
    }
    Ok(Response::new().add_attribute("method", "try_update_state_and_entries"))
}

pub fn try_cashout_yield(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
        // get balance of aust
        // yeild_amount = aust - ust_deposited 
        // send yeild_amount to owner    
    }
    Ok(Response::new().add_attribute("method", "try_cashout_yield"))
}

pub fn try_set_treasury_wallet(deps: DepsMut, info: MessageInfo, address: String) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;   
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
    let mut state = STATE.load(deps.storage)?;
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
    let mut state = STATE.load(deps.storage)?;
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
    let mut state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        state.anc_market = address;
        STATE.save(deps.storage, &state)?;
    }
    Ok(Response::new().add_attribute("method", "try_set_tier_data"))
}