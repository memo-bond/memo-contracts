use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub initializer: Addr,
    pub wallet1: Addr,
    pub wallet2: Addr,
    pub wallet3: Addr,
    pub wallet4: Addr,
    pub is_disable: bool,
    pub deduction_percentage: u16,
}

pub const STATE: Item<State> = Item::new("state");
