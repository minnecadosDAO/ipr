use cosmwasm_std::{DepsMut, Response, MessageInfo, Timestamp, StdResult};
use crate::handler::helper as ExecuteHelper;
use crate::{ContractError, state::{ENTRIES, STATE, Entry}};

pub fn try_deposit(deps: DepsMut, info: MessageInfo, entry_address: String, amount: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    // TODO: fix method of keeping track of time everywhere
    let time = Timestamp(Timestamp::seconds());
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => ExecuteHelper::some_deposit_helper(entry, amount, time),
            None => ExecuteHelper::none_deposit_helper(amount, time),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        // TODO: transfer funds from users wallet to protocol wallet
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_deposit"))
}

pub fn try_withdraw(deps: DepsMut, info: MessageInfo, entry_address: String, amount: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let time = Timestamp(Timestamp::seconds());
    let update_entry = |entry: Option<Entry>| -> Result<Entry, ContractError> {
        match entry {
            Some(entry) => ExecuteHelper::some_withdraw_helper(entry, time, amount),
            None => Err(ContractError::CannotWithdrawWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        // TODO: transfer funds from protocol wallet to users wallet
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
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
            Some(entry) => ExecuteHelper::some_claim_helper(entry),
            None => Err(ContractError::CannotClaimWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        // TODO: make sure they have claimable MIN to claim
        // transfer MIN from protocol wallet to users wallet
        ENTRIES.update(deps.storage, &valid_address, update_entry)?;
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_claim"))
}

pub fn try_sell(deps: DepsMut, info: MessageInfo, entry_address: String, amount: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    
    if info.sender == entry_address || info.sender == state.owner {
        // TODO: make sure they have amount of MIN in wallet to sell, 
        // transfer UST from protocol wallet to users wallet, 
        // transfer MIN from users wallet to protocol wallet,
    } else {
        return Err(ContractError::Unauthorized {});
    }
    Ok(Response::new().add_attribute("method", "try_sell"))
}

// TODO: admin only functions
pub fn try_update_state_and_entries(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
    }
    Ok(Response::new().add_attribute("method", "try_cashout_yield"))
}

pub fn try_cashout_yield(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
    }
    Ok(Response::new().add_attribute("method", "try_cashout_yield"))
}

pub fn try_set_protocol_wallet(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
    }
    Ok(Response::new().add_attribute("method", "try_set_protocol_wallet"))
}

pub fn try_set_treasury_wallet(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;   
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
    }
    Ok(Response::new().add_attribute("method", "try_set_treasury_wallet"))
}

pub fn try_set_reward_contract(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    if info.sender != state.owner {
        return Err(ContractError::Unauthorized {});
    } else {
        // TODO
    }
    Ok(Response::new().add_attribute("method", "try_set_reward_contract"))
}