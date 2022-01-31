use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkExecuteMsg {
    StateNumSave {
        num_to_save_start: u64,
        num_to_save_end: u64,
    },
    StateVectorSave {
        num_to_save_start: u64,
        num_to_save_end: u64,
    },
    StateNumUpdate {
        num_to_update_start: u64,
        num_to_update_end: u64,
    },
    StateVectorUpdate {
        num_to_update_start: u64,
        num_to_update_end: u64,
    },
    MapCompositeKeySave {
        first_key_start: u64,
        second_key_start: u64,
        value_start: u64,
        first_key_end: u64,
        second_key_end: u64,
        value_end: u64,
    },
    MapCompositeKeyUpdate {
        first_key_start: u64,
        second_key_start: u64,
        value_start: u64,
        first_key_end: u64,
        second_key_end: u64,
        value_end: u64,
    },
    MapVectorValueSave {
        first_key_start: u64,
        second_key_start: u64,
        value_start: u64,
        first_key_end: u64,
        second_key_end: u64,
        value_end: u64,
    },
    MapVectorValueUpdate {
        first_key_start: u64,
        second_key_start: u64,
        value_start: u64,
        first_key_end: u64,
        second_key_end: u64,
        value_end: u64,
    },
    StateExecNumsLoad {},
    StateExecVectorsLoad {},
    StateExecVectorsLoadSorted {},
    AddStakeValidators {
        number_of_validators: u64,
    },
    DelegateNValidators {
        number_of_validators: u64,
    },
    UnDelegateNValidators {
        number_of_validators: u64,
    },
    WithdrawNValidatorRewards {
        number_of_validators: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkQueryMsg {
    StateNumLoad {},
    StateVectorLoad {},
    StateVectorLoadSorted {},
    MapCompositeKeyLoad { first_key: u64, second_key: u64 },
    MapVectorValueLoad { key_to_find: u64 },
}
