use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkExecuteMsg {
    StateNumSave { num_to_save:u64 },
    StateVectorSave { num_to_save:u64 },
    StateNumUpdate { num_to_update:u64 },
    StateVectorUpdate { num_to_update:u64 },
    MapCompositeKeySave { first_key:u64,second_key:u64,value: u64 },
    MapCompositeKeyUpdate { first_key:u64,second_key:u64,value: u64 },
    MapVectorValueSave { first_key:u64,second_key:u64,value: u64 },
    MapVectorValueUpdate { first_key:u64,second_key:u64,value: u64 },
    AddValidator { validator_addr: Addr,vault_denom:String },
    StakingDelegate { validator_addr:Addr,denom:String,amount:u64 },
    StakingUnDelegate { validator_addr:Addr,denom:String,amount:u64 },
    WithdrawRewards { validator_addr:Addr },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BenchmarkQueryMsg {
    StateNumLoad{},
    StateVectorLoad{ num_to_find:u64 },
    StateVectorLoadSorted{},
    MapCompositeKeyLoad{ first_key:u64,second_key:u64 },
    MapVectorValueLoad{ key_to_find:u64 },
}


