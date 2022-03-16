use cosmwasm_std::{StdResult, Uint128, Response, coins, SubMsg, BankMsg, MessageInfo, DepsMut, Env, CosmosMsg, WasmMsg, to_binary};
use crate::handler::anchor;
use cw20::Cw20ExecuteMsg;
use crate::state::{STATE, State};
use crate::{state::{Entry, Deposit, Reward, Withdraw}, ContractError};

pub(crate) fn some_deposit_helper(state: State, deps: DepsMut, env: Env, mut entry: Entry, amount: Uint128, time: u64) -> StdResult<Entry> {
    make_deposit_and_convert_to_aust(state, deps, env, amount);    
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

pub(crate) fn none_deposit_helper(state: State, deps: DepsMut, env: Env, amount: Uint128, time: u64) -> StdResult<Entry> {
    make_deposit_and_convert_to_aust(state, deps, env, amount);
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

fn make_deposit_and_convert_to_aust(state: State, deps: DepsMut, env: Env, amount: Uint128) -> Result<Response, ContractError> {
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
    anchor::deposit_stable_msg(deps.as_ref(), &state.anc_market, "uust", amount);
    Ok(Response::new().add_attribute("method", "make_deposit_and_convert_to_aust"))
}

pub(crate) fn some_withdraw_helper(deps: DepsMut, info: MessageInfo, mut entry: Entry, time: u64, mut amount: Uint128) -> Result<Entry, ContractError> {
    convert_from_aust_and_make_withdraw(deps, info, amount);
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

    let dynamic_reward_log_clone = entry.dynamic_reward_log;
    for mut reward in dynamic_reward_log_clone {
        if reward.amount > Uint128::zero() {
            if reward.amount > amount {
                reward.amount - amount;
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
    entry.dynamic_reward_log = dynamic_reward_log_clone;
    Ok(entry)
}

fn convert_from_aust_and_make_withdraw(deps: DepsMut, info: MessageInfo, amount: Uint128) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    // swap from aust to ust
    anchor::redeem_stable_msg(deps, &state.anc_market, &state.aust_contract, amount);
    // transfer funds from contract to users wallet
    let mut response: Response = Default::default();
    let coin_amount = coins(amount.u128(), "uust");
    response.messages = vec![SubMsg::new(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coin_amount,
    })];
    Ok(Response::new().add_attribute("method", "convert_from_aust_and_make_withdraw"))
}

pub(crate) fn some_claim_helper(info: MessageInfo, mut entry: Entry) -> Result<Entry, ContractError> {
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
}