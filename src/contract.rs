#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

use cosmwasm_std::{
    to_binary, Addr, Binary, Coin, Deps, DepsMut, DistributionMsg, Env, MessageInfo, QueryResponse,
    Response, StakingMsg, StdResult, Storage, Uint128,
};

use cw2::set_contract_version;
use cw_storage_plus::U64Key;

use crate::error::ContractError;
use crate::msg::{BenchmarkExecuteMsg, BenchmarkQueryMsg, InstantiateMsg};
use crate::state::{State, MAP_COMPOSITE_KEY, MAP_VECTOR_VALUE, STATE};

mod validator_set;

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
        validator_added: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: BenchmarkExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        BenchmarkExecuteMsg::StateNumSave {
            num_to_save_start,
            num_to_save_end,
        } => state_num_save(deps, env, info, num_to_save_start, num_to_save_end),
        BenchmarkExecuteMsg::StateVectorSave {
            num_to_save_start,
            num_to_save_end,
        } => state_vector_save(deps, env, info, num_to_save_start, num_to_save_end),
        BenchmarkExecuteMsg::StateNumUpdate {
            num_to_update_start,
            num_to_update_end,
        } => state_num_update(deps, env, info, num_to_update_start, num_to_update_end),
        BenchmarkExecuteMsg::StateVectorUpdate {
            num_to_update_start,
            num_to_update_end,
        } => state_vector_update(deps, env, info, num_to_update_start, num_to_update_end),
        BenchmarkExecuteMsg::MapCompositeKeySave {
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        } => state_composite_key_save(
            deps,
            env,
            info,
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        ),
        BenchmarkExecuteMsg::MapCompositeKeyUpdate {
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        } => state_composite_key_update(
            deps,
            env,
            info,
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        ),
        BenchmarkExecuteMsg::MapVectorValueSave {
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        } => state_vector_value_save(
            deps,
            env,
            info,
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        ),
        BenchmarkExecuteMsg::StateExecNumsLoad {} => state_exec_nums_load(deps, env, info),
        BenchmarkExecuteMsg::StateExecVectorsLoad {} => state_exec_vectors_load(deps, env, info),
        BenchmarkExecuteMsg::StateExecVectorsLoadSorted {} => {
            state_exec_vectors_load_sorted(deps, env, info)
        }
        BenchmarkExecuteMsg::MapVectorValueUpdate {
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        } => state_vector_value_update(
            deps,
            env,
            info,
            first_key_start,
            second_key_start,
            value_start,
            first_key_end,
            second_key_end,
            value_end,
        ),
        BenchmarkExecuteMsg::AddStakeValidators {
            number_of_validators,
        } => add_stake_validator(deps, env, info, number_of_validators),
        BenchmarkExecuteMsg::DelegateNValidators {
            number_of_validators,
        } => delegate_n_validators(deps, env, info, number_of_validators),
        BenchmarkExecuteMsg::UnDelegateNValidators {
            number_of_validators,
        } => undelegate_n_validators(deps, env, info, number_of_validators),
        BenchmarkExecuteMsg::WithdrawNValidatorRewards {
            number_of_validators,
        } => state_n_validator_withdraw_rewards(deps, env, info, number_of_validators),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: BenchmarkQueryMsg) -> StdResult<QueryResponse> {
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

fn state_exec_nums_load(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    Ok(Response::default())
}

fn state_exec_vectors_load(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let vector_to_find = state.state_vector;
    Ok(Response::default())
}

fn state_exec_vectors_load_sorted(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    let mut vector_to_find = state.state_vector;
    vector_to_find.sort();
    Ok(Response::default())
}

fn state_exec_vectors_save_sorted(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    let mut vector_to_find = state.state_vector;
    vector_to_find.sort();
    state.state_vector = vector_to_find;
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

// only use add_stake_validator to add validators
// dont use this as this doesnt check if validator is already present or not
fn add_validator(
    storage: &mut dyn Storage,
    _env: Env,
    info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
) -> StakingMsg {
    // check if the validator exists in the blockchain

    let amount_to_stake_per_validator = Uint128::new(10);

    let funds = info.funds.first();

    // if funds.is_none() {
    //     return Err(ContractError::InsufficientFunds {});
    // already checking this inside add_stake_validators
    // }

    // if funds.unwrap().amount.lt(&amount_to_stake_per_validator) {
    //     return Err(ContractError::InsufficientFunds {});
    // passing 10 ulna as amount to stake per validator
    // }
    let mut temp = STATE.load(storage).unwrap();
    temp.validator_added.push(validator_addr.clone());
    STATE.save(storage, &temp);

    return StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: amount_to_stake_per_validator,
        },
    };

    // Ok(Response::new()
    //     .add_messages([msg])
    //     .add_attribute("method", "add_validator"))
}

fn state_staking_delegate(
    storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> StakingMsg {
    return StakingMsg::Delegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
}

fn add_stake_validator(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    number_of_validators: u64,
) -> Result<Response, ContractError> {
    let res = validator_set::return_validator_set();
    let storage = deps.storage;
    let state = STATE.load(storage)?;

    let mut validators_to_add: Vec<Addr> = vec![];
    for curr_validator in res.iter() {
        // let currs_validators=&curr_validator[..];

        let validator_addr = Addr::unchecked(curr_validator);
        if !(state.validator_added.contains(&validator_addr)) {
            validators_to_add.push(validator_addr);
        }
        if (validators_to_add.len() as u64) == number_of_validators {
            break;
        }
    }

    if (validators_to_add.len() as u64) != number_of_validators {
        return Err(ContractError::NotEnoughValidatorsFound {});
    }

    let tempcoin = Coin {
        denom: String::from("uluna"),
        amount: Uint128::new(10),
    };
    let mut infos = info.clone();
    infos.funds.push(tempcoin);
    let mut msgs = vec![];
    for validator in validators_to_add {
        msgs.push(add_validator(
            storage,
            env.clone(),
            infos.clone(),
            validator,
            String::from("uluna"),
        ));
    }

    Ok(Response::new()
        .add_messages(msgs)
        .add_attribute("method", "add_validator"))
}

fn state_staking_undelegate(
    storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
    vault_denom: String,
    amount_to_delegate: u64,
) -> StakingMsg {
    return StakingMsg::Undelegate {
        validator: validator_addr.to_string(),
        amount: Coin {
            denom: vault_denom.clone(),
            amount: Uint128::new(amount_to_delegate.into()),
        },
    };
}

fn state_num_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_save_start: u64,
    num_to_save_end: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    for i in (num_to_save_start)..(num_to_save_end + 1) {
        state.state_num = i;
    }
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

fn state_vector_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_save_start: u64,
    num_to_save_end: u64,
) -> Result<Response, ContractError> {
    let mut state = STATE.load(deps.storage)?;
    for i in (num_to_save_start)..(num_to_save_end + 1) {
        state.state_vector.push(i);
    }
    STATE.save(deps.storage, &state);
    Ok(Response::default())
}

fn state_num_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_update_start: u64,
    num_to_update_end: u64,
) -> Result<Response, ContractError> {
    for x in num_to_update_start..(num_to_update_end + 1) {
        STATE.update(deps.storage, |mut s| -> StdResult<_> {
            s.state_num = x;
            Ok(s)
        })?;
    }
    Ok(Response::default())
}

fn state_vector_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    num_to_update_start: u64,
    num_to_update_end: u64,
) -> Result<Response, ContractError> {
    for x in num_to_update_start..(num_to_update_end + 1) {
        STATE.update(deps.storage, |mut s| -> StdResult<_> {
            s.state_vector.push(x);
            Ok(s)
        })?;
    }
    Ok(Response::default())
}

fn state_composite_key_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key_start: u64,
    second_key_start: u64,
    value_start: u64,
    first_key_end: u64,
    second_key_end: u64,
    value_end: u64,
) -> Result<Response, ContractError> {
    // make sure to save the value at first_key first before updating
    // checking by loading the map will add the gas fees of loading too

    let mut first_key = first_key_start;
    let mut second_key = second_key_start;
    for x in value_start..(value_end + 1) {
        MAP_COMPOSITE_KEY.update(
            deps.storage,
            (U64Key::new(first_key), U64Key::new(second_key)),
            |_v| -> StdResult<Uint128> { Ok(x.into()) },
        )?;
        first_key = first_key + 1;
        second_key = second_key + 1;
    }

    Ok(Response::default())
}

fn state_composite_key_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key_start: u64,
    second_key_start: u64,
    value_start: u64,
    first_key_end: u64,
    second_key_end: u64,
    value_end: u64,
) -> Result<Response, ContractError> {
    let mut first_key = first_key_start;
    let mut second_key = second_key_start;
    for x in value_start..(value_end + 1) {
        MAP_COMPOSITE_KEY.save(
            deps.storage,
            (U64Key::new(first_key), U64Key::new(second_key)),
            &(x.into()),
        )?;
        first_key = first_key + 1;
        second_key = second_key + 1;
    }

    Ok(Response::default())
}

fn state_vector_value_save(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key_start: u64,
    second_key_start: u64,
    value_start: u64,
    first_key_end: u64,
    second_key_end: u64,
    value_end: u64,
) -> Result<Response, ContractError> {
    let mut first_key = first_key_start;
    let mut second_key = second_key_start;

    for x in value_start..(value_end + 1) {
        let mut vec_to_save = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
        vec_to_save.push((second_key, x.into()));
        MAP_VECTOR_VALUE.save(deps.storage, U64Key::new(first_key), &vec_to_save)?;
        first_key = first_key + 1;
        second_key = second_key + 1;
    }

    Ok(Response::default())
}

fn state_vector_value_update(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    first_key_start: u64,
    second_key_start: u64,
    value_start: u64,
    first_key_end: u64,
    second_key_end: u64,
    value_end: u64,
) -> Result<Response, ContractError> {
    // save the value in map before updating

    let mut first_key = first_key_start;
    let mut second_key = second_key_start;

    for x in value_start..(value_end + 1) {
        let mut vec_to_update = MAP_VECTOR_VALUE.load(deps.storage, U64Key::new(first_key))?;
        vec_to_update.push((second_key, x.into()));
        MAP_VECTOR_VALUE.update(
            deps.storage,
            U64Key::new(first_key),
            |_v| -> StdResult<Vec<(u64, Uint128)>> { Ok(vec_to_update) },
        )?;
        first_key = first_key + 1;
        second_key = second_key + 1;
    }

    Ok(Response::default())
}

fn state_withdraw_rewards(
    storage: &mut dyn Storage,
    _env: Env,
    _info: MessageInfo,
    validator_addr: Addr,
) -> DistributionMsg {
    // make sure the validator is added first
    return DistributionMsg::WithdrawDelegatorReward {
        validator: validator_addr.to_string(),
    };
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

fn delegate_n_validators(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    number_of_validators: u64,
) -> Result<Response, ContractError> {
    let mut s = STATE.load(deps.storage)?;
    let mut currnum = 0;
    let storage = deps.storage;
    let mut msgs = vec![];
    for curr_validator in s.validator_added.iter() {
        msgs.push(state_staking_delegate(
            storage,
            env.clone(),
            info.clone(),
            curr_validator.clone(),
            "uluna".to_string(),
            10,
        ));
        currnum += 1;
        if currnum == number_of_validators {
            break;
        }
    }

    Ok(Response::new().add_messages(msgs))
}

fn undelegate_n_validators(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    number_of_validators: u64,
) -> Result<Response, ContractError> {
    let mut s = STATE.load(deps.storage)?;
    let mut currnum = 0;
    let storage = deps.storage;
    let mut msgs = vec![];
    for curr_validator in s.validator_added.iter() {
        msgs.push(state_staking_undelegate(
            storage,
            env.clone(),
            info.clone(),
            curr_validator.clone(),
            "uluna".to_string(),
            10,
        ));
        currnum += 1;
        if currnum == number_of_validators {
            break;
        }
    }

    Ok(Response::new().add_messages(msgs))
}

fn state_n_validator_withdraw_rewards(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    number_of_validators: u64,
) -> Result<Response, ContractError> {
    let mut s = STATE.load(deps.storage)?;
    let mut currnum = 0;
    let storage = deps.storage;
    let mut msgs = vec![];
    for curr_validator in s.validator_added.iter() {
        msgs.push(state_withdraw_rewards(
            storage,
            env.clone(),
            info.clone(),
            curr_validator.clone(),
        ));
        currnum += 1;
        if currnum == number_of_validators {
            break;
        }
    }

    Ok(Response::new().add_messages(msgs))
}
