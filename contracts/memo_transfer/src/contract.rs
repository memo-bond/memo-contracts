#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    coins, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    Uint128,
};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TransferBankMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:memo-transfer";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let wallet1_addr = deps.api.addr_validate(&msg.wallet1)?;
    let wallet2_addr = deps.api.addr_validate(&msg.wallet2)?;
    let wallet3_addr = deps.api.addr_validate(&msg.wallet3)?;
    let wallet4_addr = deps.api.addr_validate(&msg.wallet4)?;

    if msg.deduction_percentage >= 100 {
        return Err(ContractError::DeductionPercentageExceed {});
    }

    let state = State {
        initializer: info.sender.clone(),
        wallet1: wallet1_addr,
        wallet2: wallet2_addr,
        wallet3: wallet3_addr,
        wallet4: wallet4_addr,
        is_disable: false,
        deduction_percentage: msg.deduction_percentage,
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("initializer", info.sender)
        .add_attribute("deductionRate", msg.deduction_percentage.to_string())
        .add_attribute("wallet1Addr", msg.wallet1)
        .add_attribute("wallet2Addr", msg.wallet2)
        .add_attribute("wallet3Addr", msg.wallet3)
        .add_attribute("wallet4Addr", msg.wallet4))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;
    if state.is_disable {
        return Err(ContractError::Disabled {});
    }

    match msg {
        ExecuteMsg::Transfer {
            recipient,
            denom,
        } => try_transfer(deps, env, info, recipient, Uint128::new(100000_u128), denom),
        ExecuteMsg::DisableContract {} => try_disable(deps),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetContractState {} => to_binary(&STATE.load(deps.storage)?),
        QueryMsg::GetBalance { address } => to_binary(&deps.querier.query_all_balances(address)?),
    }
}

pub fn try_disable(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut s| -> StdResult<_> {
        s.is_disable = true;
        Ok(s)
    })?;
    Ok(Response::new()
        .add_attribute("method", "try_disable")
        .add_attribute("message", "success"))
}

pub fn try_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    recipient: String,
    amount: Uint128,
    denom: String,
) -> Result<Response, ContractError> {
    let state = STATE.load(deps.storage)?;

    let recipient_amount = amount
        .checked_mul(Uint128::new((100 - state.deduction_percentage) as u128))
        .unwrap();
    let deduction_amount = amount - recipient_amount;
    let member_deduction_amount = deduction_amount.checked_div(Uint128::new(4_u128)).unwrap();
    let sender_addr: String = info.sender.into_string();
    let mut messages: Vec<CosmosMsg<TransferBankMsg>> = vec![];

    messages.push(
        TransferBankMsg {
            from_address: sender_addr.clone(),
            to_address: recipient.clone(),
            amount: coins(recipient_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        TransferBankMsg {
            from_address: sender_addr.clone(),
            to_address: state.wallet1.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        TransferBankMsg {
            from_address: sender_addr.clone(),
            to_address: state.wallet2.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        TransferBankMsg {
            from_address: sender_addr.clone(),
            to_address: state.wallet3.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        TransferBankMsg {
            from_address: sender_addr.clone(),
            to_address: state.wallet4.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    Ok(Response::new()
        // .add_messages(messages)
        .add_attribute("method", "try_transfer")
        .add_attribute(
            "deductionPercentage",
            state.deduction_percentage.to_string().add("%"),
        )
        .add_attribute("transfer.denom", denom)
        .add_attribute("transfer.sender", sender_addr)
        .add_attribute("transfer.recipient", recipient)
        .add_attribute("transfer.amount", amount.to_string())
        .add_attribute("transfer.block_time", env.block.time.seconds().to_string()))
}
