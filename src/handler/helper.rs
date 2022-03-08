use cosmwasm_std::{Timestamp, StdResult};

use crate::{state::{Entry, Deposit, Reward, Withdraw}, ContractError};

pub(crate) fn some_deposit_helper(entry: Entry, amount: u64, time: Timestamp) -> StdResult<Entry> {
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

pub(crate) fn none_deposit_helper(amount: u64, time: Timestamp) -> StdResult<Entry> {
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
        claimable_reward: 0, 
        ust_deposited: amount, 
        averaged_reward_rate: 0.0,
        ust_deposit_log: vec![deposit], 
        ust_withdraw_log: vec![], 
        dynamic_reward_log: vec![reward] 
    };
    Ok(entry)
}

pub(crate) fn some_withdraw_helper(entry: Entry, time: Timestamp, amount: u64) -> Result<Entry, ContractError> {
    if entry.ust_deposited == 0 {
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
        if reward.amount > 0 {
            if reward.amount > amount {
                reward.amount - amount;
                break
            } else if reward.amount == amount {
                reward.amount = 0;
                break
            } else if reward.amount < amount {
                amount -= reward.amount;
                reward.amount = 0;
            }
        }
    }

    Ok(entry)
}

pub(crate) fn some_claim_helper(entry: Entry) -> Result<Entry, ContractError> {
    entry.claimable_reward = 0;
    Ok(entry)
}