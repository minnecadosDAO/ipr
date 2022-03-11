use cosmwasm_std::{StdResult, Uint128, Response, Deps, CanonicalAddr, CosmosMsg, to_binary, WasmMsg, coins, SubMsg, BankMsg, MessageInfo};
use schemars::JsonSchema;
use serde::{Serialize, Deserialize};

use crate::{state::{Entry, Deposit, Reward, Withdraw}, ContractError};

/* 
use cosmwasm_std::*;
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
*/

pub(crate) fn some_deposit_helper(mut entry: Entry, amount: Uint128, time: u64) -> StdResult<Entry> {
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

pub(crate) fn none_deposit_helper(amount: Uint128, time: u64) -> StdResult<Entry> {
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

pub(crate) fn some_withdraw_helper(mut entry: Entry, time: u64, mut amount: Uint128) -> Result<Entry, ContractError> {
    if entry.ust_deposited == Uint128::zero() {
        return Err(ContractError::CannotWithdrawBalanceZero {});
    }
    if (amount <= entry.ust_deposited){
        entry.ust_deposited -= amount;
    } else {
        return Err(ContractError::CannotWithdrawGreaterThanBalance {});
    }
    let withdraw = Withdraw {
        amount: amount,
        time: time,
    };
    entry.ust_withdraw_log.push(withdraw);

    for reward in entry.dynamic_reward_log {
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

    Ok(entry)
}

pub(crate) fn some_claim_helper(info: MessageInfo, mut entry: Entry) -> Result<Entry, ContractError> {
    if entry.claimable_reward == Uint128::zero() {
        return Err(ContractError::Unauthorized {});
    }
    // transfer MIN from treasury wallet to users wallet
    // will have to send MIN/Perma to this smart contract to give out
    let mut response: Response = Default::default();
    let coin_amount = coins(entry.claimable_reward.u128(), "umin");
    response.messages = vec![SubMsg::new(BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: coin_amount,
    })];
    entry.claimable_reward = Uint128::zero();
    Ok(entry)
}

/* 
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

pub fn deposit_stable_msg(
    deps: Deps,
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

pub fn redeem_stable_msg(
    deps: Deps,
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
*/