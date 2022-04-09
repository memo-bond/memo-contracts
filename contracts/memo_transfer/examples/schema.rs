use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};

use memo_contract::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, CustomMsg};
use memo_contract::state::State;

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("../contracts/schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(QueryMsg), &out_dir);
    export_schema(&schema_for!(State), &out_dir);
    export_schema(&schema_for!(CustomMsg), &out_dir);
}
