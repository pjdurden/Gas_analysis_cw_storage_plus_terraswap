use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

use cw_storage_plus::{Item, Map, U64Key};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub state_num: u64,
    pub state_vector: Vec<u64>,
    pub validator_added: Vec<Addr>,
}

pub const STATE: Item<State> = Item::new("state");

pub const MAP_COMPOSITE_KEY: Map<(U64Key, U64Key), Uint128> = Map::new("map_composite_key");
pub const MAP_VECTOR_VALUE: Map<U64Key, Vec<(u64, Uint128)>> = Map::new("map_vector_value");
