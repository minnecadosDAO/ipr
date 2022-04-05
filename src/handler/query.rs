use cosmwasm_std::{Deps, StdResult};

use crate::{msg::{EntryResponse, StateResponse }, state::{STATE, ENTRIES}};

pub fn query_entry(deps: Deps, entry_address: String) -> StdResult<EntryResponse> {
    let valid_address = deps.api.addr_validate(&entry_address)?;
    let entry = ENTRIES.load(deps.storage, &valid_address)?;
    Ok(EntryResponse { 
        claimable_reward: entry.claimable_reward,
        ust_deposited: entry.ust_deposited,
        averaged_reward_rate: entry.averaged_reward_rate,
        ust_deposit_log: entry.ust_deposit_log,
        ust_withdraw_log: entry.ust_withdraw_log,
        dynamic_reward_log: entry.dynamic_reward_log,
    })
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(StateResponse { 
        owner: state.owner,
        treasury_wallet: state.treasury_wallet,
        reward_contract: state.reward_contract,
        ust_deposited: state.ust_deposited,
        sellback_price: state.sellback_price,
        anc_market: state.anc_market,
        aust_contract: state.aust_contract,
        tier0rate: state.tier0rate,
        tier0time: state.tier0time,
        tier1rate: state.tier1rate,
        tier1time: state.tier1time,
        tier2rate: state.tier2rate,
        tier2time: state.tier2time,
        tier3rate: state.tier3rate,
        tier3time: state.tier3time,
    })
}