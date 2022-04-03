use cosmwasm_std::{Coin, CosmosMsg};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub wallet1: String,
    pub wallet2: String,
    pub wallet3: String,
    pub wallet4: String,
    pub deduction_percentage: u16,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        denom: String,
    },
    DisableContract {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetContractState {},
    GetBalance { address: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TransferBankMsg {
    pub from_address: String,
    pub to_address: String,
    pub amount: Vec<Coin>,
}

impl From<TransferBankMsg> for CosmosMsg<TransferBankMsg> {
    fn from(original: TransferBankMsg) -> Self {
        CosmosMsg::Custom(original)
    }
}

// impl Into<CosmosMsg<TransferBankMsg>> for TransferBankMsg {
//     fn into(self) -> CosmosMsg<TransferBankMsg> {
//         CosmosMsg::Custom(self)
//     }
// }
