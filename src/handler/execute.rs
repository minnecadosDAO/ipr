use cosmwasm_std::{DepsMut, Response, MessageInfo, StdResult, Uint128, Env, SubMsg, BankMsg, coins, CanonicalAddr, to_binary, CosmosMsg, WasmMsg, Deps};
use crate::{ContractError, state::{ENTRIES, STATE, Entry}};
use crate::handler::anchor;
use cw20::Cw20ExecuteMsg;
use crate::{state::{Deposit, Reward, Withdraw}};

pub fn try_deposit(deps: DepsMut, info: MessageInfo, env: Env, entry_address: String, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let time = env.block.time.seconds();
    
    let upsert_entry = |entry: Option<Entry>| -> StdResult<Entry> {
        match entry {
            Some(entry) => {some_deposit_helper(entry, amount, time)},
            None => {none_deposit_helper(amount, time)},
        }
    };
    
    if info.sender == entry_address || info.sender == state.owner {
        make_deposit_and_convert_to_aust(deps.as_ref(), env, amount)?;
        ENTRIES.update(deps.storage, &valid_address, upsert_entry)?;
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
            Some(entry) => some_withdraw_helper(entry, time, amount),
            None => Err(ContractError::CannotWithdrawWithoutDeposit {}),
        }
    };

    if info.sender == entry_address || info.sender == state.owner {
        convert_from_aust_and_make_withdraw(deps.as_ref(), info, amount)?;
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
            Some(mut entry) => {
                if entry.claimable_reward == Uint128::zero() {
                    return Err(ContractError::Unauthorized {});
                }
                // transfer MIN from treasury wallet to users wallet
                // will have to send MIN to this smart contract to give out
                let mut response: Response = Default::default();
                let coin_amount = coins(entry.claimable_reward.u128(), "umin");
                response.messages = vec![SubMsg::new(BankMsg::Send {
                    to_address: info.sender.to_string(),
                    amount: coin_amount,
                })];
                entry.claimable_reward = Uint128::zero();
                Ok(entry)
            },
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

// TODO
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

// TODO
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

// helpers
fn some_deposit_helper(mut entry: Entry, amount: Uint128, time: u64) -> StdResult<Entry> {
    entry.ust_deposited += amount;
    let deposit = Deposit {
        amount: amount,
        time: time,
    };
    entry.ust_deposit_log.push(deposit);
    let reward = Reward {
        amount: amount,
        time: time,
        reward_tier: 0,
    };
    entry.dynamic_reward_log.push(reward);
    Ok(entry)
}

fn none_deposit_helper(amount: Uint128, time: u64) -> StdResult<Entry> {
    let deposit = Deposit {
        amount: amount,
        time: time,
    };
    let reward = Reward {
        amount: amount,
        time: time,
        reward_tier: 0,
    };
    let entry = Entry {
        claimable_reward: Uint128::zero(), 
        ust_deposited: amount, 
        averaged_reward_rate: 0.0,
        ust_deposit_log: vec![deposit], 
        ust_withdraw_log: vec![], 
        dynamic_reward_log: vec![reward] 
    };
    Ok(entry)
}

fn make_deposit_and_convert_to_aust(deps: Deps, env: Env, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    // transfer funds from user to protocol
    let mut response: Response = Default::default();
    let coin_amount = coins(amount.u128(), "uust");
    // I think including cw20 has messed with dependencies bringing in newest alpha version of cosmwasm_std
    response.messages = vec![SubMsg::new(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: env.contract.address.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {amount: amount, contract:env.contract.address.to_string(), msg: to_binary("data")?})?,
        funds: coin_amount,
    }))];

    // swap ust for aust
    anchor::deposit_stable_msg(deps, &state.anc_market, "uust", amount)?;
    Ok(Response::new().add_attribute("method", "make_deposit_and_convert_to_aust"))
}

fn some_withdraw_helper(mut entry: Entry, time: u64, mut amount: Uint128) -> Result<Entry, ContractError> {
    if entry.ust_deposited == Uint128::zero() {
        return Err(ContractError::CannotWithdrawBalanceZero {});
    }
    if amount <= entry.ust_deposited {
        entry.ust_deposited -= amount;
    } else {
        return Err(ContractError::CannotWithdrawGreaterThanBalance {});
    }
    let withdraw = Withdraw {
        amount: amount,
        time: time,
    };
    entry.ust_withdraw_log.push(withdraw);

    //let mut dynamic_reward_log_clone = entry.dynamic_reward_log;
    for reward in &mut entry.dynamic_reward_log {
        let reward: &mut Reward = reward;
        if reward.amount > Uint128::zero() {
            if reward.amount > amount {
                reward.amount -= amount;
                break
            } else if reward.amount == amount {
                reward.amount = Uint128::zero();
                break
            } else if reward.amount < amount {
                amount -= reward.amount;
                reward.amount = Uint128::zero();
            }
        }
    }
    //entry.dynamic_reward_log = dynamic_reward_log_clone;
    Ok(entry)
}

fn convert_from_aust_and_make_withdraw(deps: Deps, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    // swap from aust to ust
    anchor::redeem_stable_msg(deps, &state.anc_market, &state.aust_contract, amount)?;
    // transfer funds from contract to users wallet
    let mut response: Response = Default::default();
    let coin_amount = coins(amount.u128(), "uust");
    response.messages = vec![SubMsg::new(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coin_amount,
    })];
    Ok(Response::new().add_attribute("method", "convert_from_aust_and_make_withdraw"))
}