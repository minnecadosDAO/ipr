use cosmwasm_std::{DepsMut, Response, MessageInfo, Timestamp, StdResult};

use crate::{ContractError, state::{ENTRIES, STATE, Entry, Deposit, Reward, Withdraw}};

pub fn try_deposit(deps: DepsMut, info: MessageInfo, entry_address: String, amount: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let entry = ENTRIES.load(deps.storage, &valid_address)?;
    let time_now = Timestamp(Timestamp::seconds());
    
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => Ok(Entry {
                claimable_reward: entry.claimable_reward, 
                ust_deposited: entry.ust_deposited + amount, 
                averaged_reward_rate: entry.averaged_reward_rate,
                ust_deposit_log: entry.ust_deposit_log.concat(vec![Deposit{
                    amount: amount,
                    time: time_now,
                }]), 
                ust_withdraw_log: entry.ust_withdraw_log, 
                dynamic_reward_log: entry.dynamic_reward_log.concat(vec![Reward{
                    amount: amount,
                    time: time_now,
                    reward_tier: 0,
                }]), 
            }),
            None => Ok(Entry { 
                claimable_reward: 0, 
                ust_deposited: amount, 
                averaged_reward_rate: 0.0,
                ust_deposit_log: vec![Deposit{
                    amount: amount,
                    time: time_now,
                }], 
                ust_withdraw_log: vec![], 
                dynamic_reward_log: vec![Reward{
                    amount: amount,
                    time: time_now,
                    reward_tier: 0,
                }] 
            }),
        }
    };
    
    if info.sender != entry_address {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        } else {
            // TODO: transfer funds from users wallet to protocol wallet
            ENTRIES.update(deps.storage, &entry_address, update_entry)?;
        }
    } else {
        // TODO: transfer funds from users wallet to protocol wallet
        ENTRIES.update(deps.storage, &entry_address, update_entry)?;
    }
    Ok(Response::new().add_attribute("method", "try_deposit"))
}

pub fn try_withdraw(deps: DepsMut, info: MessageInfo, entry_address: String, amount: u64) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let entry = ENTRIES.load(deps.storage, &valid_address)?;
    let time_now = Timestamp(Timestamp::seconds());
    
    // TODO: need to update dynamic reward log on deposits and withdraws
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => Ok(Entry {
                claimable_reward: entry.claimable_reward, 
                ust_deposited: entry.ust_deposited - amount, 
                averaged_reward_rate: entry.averaged_reward_rate,
                ust_deposit_log: entry.ust_deposit_log, 
                ust_withdraw_log: entry.ust_withdraw_log.concat(vec![Withdraw{
                    amount: amount,
                    time: time_now,
                }]), 
                dynamic_reward_log: entry.dynamic_reward_log, 
            }),
            None => Ok(Entry { 
                claimable_reward: 0, 
                ust_deposited: amount, 
                averaged_reward_rate: 0.0,
                ust_deposit_log: vec![],
                ust_withdraw_log: vec![Withdraw{
                    amount: amount,
                    time: time_now,
                }], 
                dynamic_reward_log: vec![Reward{
                    amount: amount,
                    time: time_now,
                    reward_tier: 0,
                }] 
            }),
        }
    };
    
    if info.sender != entry_address {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        } else {
            // TODO: transfer funds from protocol wallet to users wallet
            ENTRIES.update(deps.storage, &entry_address, update_entry)?;
        }
    } else {
        // TODO: transfer funds from protocol wallet to users wallet
        ENTRIES.update(deps.storage, &entry_address, update_entry)?;
    }
    Ok(Response::new().add_attribute("method", "try_deposit"))
}

pub fn try_claim(deps: DepsMut, info: MessageInfo, entry_address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let entry = ENTRIES.load(deps.storage, &valid_address)?;
    let time_now = Timestamp(Timestamp::seconds());
    
    let update_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => Ok(Entry {
                claimable_reward: 0, 
                ust_deposited: entry.ust_deposited, 
                averaged_reward_rate: entry.averaged_reward_rate,
                ust_deposit_log: entry.ust_deposit_log, 
                ust_withdraw_log: entry.ust_withdraw_log, 
                dynamic_reward_log: entry.dynamic_reward_log, 
            }),
            None => Ok(Entry { 
                claimable_reward: 0, 
                ust_deposited: 0, 
                averaged_reward_rate: 0.0,
                ust_deposit_log: vec![],
                ust_withdraw_log: vec![], 
                dynamic_reward_log: vec![],
            }),
        }
    };
    
    if info.sender != entry_address {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        } else {
            // TODO: transfer MIN from protocol wallet to users wallet, make sure they have claimable MIN to claim
            ENTRIES.update(deps.storage, &entry_address, update_entry)?;
        }
    } else {
        // TODO: transfer MIN from protocol wallet to users wallet, make sure they have claimable MIN to claim
        ENTRIES.update(deps.storage, &entry_address, update_entry)?;
    }
    Ok(Response::new().add_attribute("method", "try_claim"))
}

pub fn try_sell(deps: DepsMut, info: MessageInfo, entry_address: String) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let entry = ENTRIES.load(deps.storage, &valid_address)?;
    
    if info.sender != entry_address {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        } else {
            // TODO: make sure they have MIN to sell, 
            // transfer UST from protocol wallet to users wallet, 
            // transfer MIN from users wallet to protocol wallet
        }
    } else {
        // TODO: make sure they have MIN to sell, 
        // transfer UST from protocol wallet to users wallet, 
        // transfer MIN from users wallet to protocol wallet
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