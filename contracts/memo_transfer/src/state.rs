use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{Bucket, bucket};
use cw_storage_plus::Item;

pub const PREFIX_BALANCE: &[u8] = b"balance";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub initializer: Addr,
    pub wallet1: Addr,
    pub wallet2: Addr,
    pub wallet3: Addr,
    pub wallet4: Addr,
    pub is_disable: bool,
    pub deduction_percentage: Uint128,
}

pub fn balances(storage: &mut dyn Storage) -> Bucket<Uint128> {
    bucket(storage, PREFIX_BALANCE)
}

pub const STATE: Item<State> = Item::new("state");
