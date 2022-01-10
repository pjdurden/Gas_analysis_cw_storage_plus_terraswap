#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, DistributionMsg, Env, MessageInfo, Response,
    StakingMsg, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_storage_plus::U64Key;

use crate::error::ContractError;
use crate::msg::{BenchmarkExecuteMsg, BenchmarkQueryMsg, InstantiateMsg};
use crate::state::{State, MAP_COMPOSITE_KEY, MAP_VECTOR_VALUE, STATE};

// use terra_cosmwasm::TerraQuerier;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:gas-fees-benchmark";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        state_num: 1,
        state_vector: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: BenchmarkExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        BenchmarkExecuteMsg::StateNumSave { num_to_save } => {
            state_num_save(deps, _env, info, num_to_save)
        }
        BenchmarkExecuteMsg::StateVectorSave { num_to_save } => {
            state_vector_save(deps, _env, info, num_to_save)
        }
        BenchmarkExecuteMsg::StateNumUpdate { num_to_update } => {
            state_num_update(deps, _env, info, num_to_update)
        }
        BenchmarkExecuteMsg::StateVectorUpdate { num_to_update } => {
            state_vector_update(deps, _env, info, num_to_update)
        }
        BenchmarkExecuteMsg::MapCompositeKeySave {
            first_key,
            second_key,
            value,
        } => state_composite_key_save(deps, _env, info, first_key, second_key, value),
        BenchmarkExecuteMsg::MapCompositeKeyUpdate {
            first_key,
            second_key,
            value,
        } => state_composite_key_update(deps, _env, info, first_key, second_key, value),
        BenchmarkExecuteMsg::MapVectorValueSave {
            first_key,
            second_key,
            value,
        } => state_vector_value_save(deps, _env, info, first_key, second_key, value),
        BenchmarkExecuteMsg::MapVectorValueUpdate {
            first_key,
            second_key,
            value,
        } => state_vector_value_update(deps, _env, info, first_key, second_key, value),
        BenchmarkExecuteMsg::AddValidator {
            validator_addr,
            vault_denom,
        } => add_validator(deps, _env, info, validator_addr, vault_denom),
        BenchmarkExecuteMsg::StakingDelegate {
            validator_addr,
            denom,
            amount,
        } => state_staking_delegate(deps, _env, info, validator_addr, denom, amount),
        BenchmarkExecuteMsg::StakingUnDelegate {
            validator_addr,
            denom,
            amount,
        } => state_staking_undelegate(deps, _env, info, validator_addr, denom, amount),
        BenchmarkExecuteMsg::WithdrawRewards { validator_addr } => {
            state_withdraw_rewards(deps, _env, info, validator_addr)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: BenchmarkQueryMsg) -> StdResult<Binary> {
    match msg {
        BenchmarkQueryMsg::StateNumLoad {} => to_binary(&query_state_num_load(deps, _env)?),
        BenchmarkQueryMsg::StateVectorLoad {} => to_binary(&query_state_vector_load(deps, _env)?),
        BenchmarkQueryMsg::StateVectorLoadSorted {} => {
            to_binary(&query_state_vector_load_sorted(deps, _env)?)
        }
        BenchmarkQueryMsg::MapCompositeKeyLoad {
            first_key,
            second_key,
        } => to_binary(&query_map_composite_key_load(
            deps, _env, first_key, second_key,
        )?),
        BenchmarkQueryMsg::MapVectorValueLoad { key_to_find } => {
            to_binary(&query_map_vector_value_load(deps, _env, key_to_find)?)
        }
    }
}

fn add_validator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
) -> Result<Response, ContractError> {
    // check if the validator exists in the blockchain
    if deps
        .querier
        .query_validator(validator_addr.clone())?
        .is_none()
    {
        return Err(ContractError::ValidatorDoesNotExist {});
    }

    let amount_to_stake_per_validator = Uint128::new(10);

    let funds = info.funds.first();
    if funds.is_none() {
        return Err(ContractError::InsufficientFunds {});
    }

    if funds.unwrap().amount.lt(&amount_to_stake_per_validator) {
        return Err(ContractError::InsufficientFunds {});
    }

    let msg = StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: amount_to_stake_per_validator,
        },
    };

    Ok(Response::new()
        .add_messages([msg])
        .add_attribute("method", "add_validator"))
}

fn state_staking_delegate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> Result<Response, ContractError> {
    let msg = StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
    Ok(Response::new().add_messages([msg]))
}

fn state_staking_undelegate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> Result<Response, ContractError> {
    let msg = StakingMsg::Undelegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
    Ok(Response::new().add_messages([msg]))
}

fn state_num_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_save: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    state.state_num = num_to_save;
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

fn state_vector_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_save: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    state.state_vector.push(num_to_save);
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

fn state_num_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_update: u64,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut s| -> StdResult<_> {
        s.state_num = num_to_update;
        Ok(s)
    })?;
    Ok(Response::default())
}

fn state_vector_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_update: u64,
) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut s| -> StdResult<_> {
        s.state_vector.push(num_to_update);
        Ok(s)
    })?;
    Ok(Response::default())
}

fn state_composite_key_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key: u64,
    second_key: u64,
    value: u64,
) -> Result<Response, ContractError> {
    // make sure to save the value at first_key first before updating
    // checking by loading the map will add the gas fees of loading too
    MAP_COMPOSITE_KEY.update(
        deps.storage,
        (U64Key::new(first_key), U64Key::new(second_key)),
        |_v| -> StdResult<Uint128> { Ok(value.into()) },
    )?;
    Ok(Response::default())
}

fn state_composite_key_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key: u64,
    second_key: u64,
    value: u64,
) -> Result<Response, ContractError> {
    MAP_COMPOSITE_KEY.save(
        deps.storage,
        (U64Key::new(first_key), U64Key::new(second_key)),
        &(value.into()),
    )?;

    Ok(Response::default())
}

fn state_vector_value_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key: u64,
    _second_key: u64,
    value: u64,
) -> Result<Response, ContractError> {
    let mut vec_to_save = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
    vec_to_save.push((first_key, value.into()));
    MAP_VECTOR_VALUE.save(deps.storage, U64Key::new(first_key), &vec_to_save)?;
    Ok(Response::default())
}

fn state_vector_value_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key: u64,
    _second_key: u64,
    value: u64,
) -> Result<Response, ContractError> {
    // save the value in map before updating
    let mut vec_to_update = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
    vec_to_update.push((first_key, value.into()));
    MAP_VECTOR_VALUE.update(
        deps.storage,
        U64Key::new(first_key),
        |_v| -> StdResult<Vec<(u64, Uint128)>> { Ok(vec_to_update) },
    )?;
    Ok(Response::default())
}

fn state_withdraw_rewards(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
) -> Result<Response, ContractError> {
    // make sure the validator is added first
    let msg = DistributionMsg::WithdrawDelegatorReward {
        validator: validator_addr.to_string(),
    };
    Ok(Response::new().add_messages([msg]))
}

fn query_state_num_load(deps: Deps, _env: Env) -> StdResult<u64> {
    let state = STATE.load(deps.storage)?;
    Ok(state.state_num)
}

fn query_state_vector_load(deps: Deps, _env: Env) -> StdResult<Vec<u64>> {
    let state = STATE.load(deps.storage)?;
    Ok(state.state_vector)
}

fn query_state_vector_load_sorted(deps: Deps, _env: Env) -> StdResult<Vec<u64>> {
    let state = STATE.load(deps.storage)?;
    let mut vec_to_sort = state.state_vector;
    vec_to_sort.sort();
    Ok(vec_to_sort)
}

fn query_map_composite_key_load(
    deps: Deps,
    _env: Env,
    first_key: u64,
    second_key: u64,
) -> StdResult<Uint128> {
    let value_to_find = MAP_COMPOSITE_KEY.load(
        deps.storage,
        (U64Key::new(first_key), U64Key::new(second_key)),
    )?;
    Ok(value_to_find)
}

fn query_map_vector_value_load(
    deps: Deps,
    _env: Env,
    num_to_find: u64,
) -> StdResult<Vec<(u64, Uint128)>> {
    let value_to_find = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(num_to_find))?;
    Ok(value_to_find)
}
