use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    // Change 'count' to whatever you want
    // Changed from: 'pub count: i32,'
    pub date: i32,
    pub owner: Addr,
    // Here's the vector
    pub scores: Vec<(Addr, u16)>
}

// I renamed this from 'STATE' TO 'STORAGE' to avoid confusion
pub const STORAGE: Item<State> = Item::new("state");
