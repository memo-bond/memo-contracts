#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{coins, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, StdError};
use cw2::set_contract_version;
use std::ops::Add;

use crate::error::ContractError;
use crate::msg::{CustomMsg, ExecuteMsg, InstantiateMsg, QueryMsg};
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

    if msg.deduction_percentage.gt(&Uint128::from(100_u128)) {
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
) -> Result<Response<CustomMsg>, ContractError> {
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

pub fn try_disable(deps: DepsMut) -> Result<Response<CustomMsg>, ContractError> {
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
) -> Result<Response<CustomMsg>, ContractError> {
    let state = STATE.load(deps.storage)?;
    // println!("amount: {:?}", amount);

    let recipient_percent = Uint128::from(100_u128)
        .checked_sub(state.deduction_percentage)
        .map_err(StdError::overflow)?;

    let recipient_amount =
        amount.checked_mul(recipient_percent)
              .map_err(StdError::overflow)?
            .checked_div(Uint128::from(100_u128))
            .map_err(StdError::divide_by_zero)?;

    let deduction_amount = amount
        .checked_sub(recipient_amount)
        .map_err(StdError::overflow)?;

    let member_deduction_amount = deduction_amount.checked_div(Uint128::from(4_u128))
                                                  .map_err(StdError::divide_by_zero)?;

    // println!("recipient amount: {:?}", recipient_amount);
    // println!("deduction: {:?}", deduction_amount);
    // println!("member amount: {:?}", member_deduction_amount);
    let sender_addr: String = info.sender.into_string();
    let mut messages: Vec<CosmosMsg<CustomMsg>> = vec![];

    messages.push(
        CustomMsg::Transfer {
            from_address: sender_addr.clone(),
            to_address: recipient.clone(),
            amount: coins(recipient_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        CustomMsg::Transfer {
            from_address: sender_addr.clone(),
            to_address: state.wallet1.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        CustomMsg::Transfer {
            from_address: sender_addr.clone(),
            to_address: state.wallet2.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        CustomMsg::Transfer {
            from_address: sender_addr.clone(),
            to_address: state.wallet3.into_string(),
            amount: coins(member_deduction_amount.u128(), denom.clone()),
        }
        .into(),
    );

    messages.push(
        CustomMsg::Transfer {
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
           .add_attribute("transfer.recipient.percent", recipient_percent.to_string())
           .add_attribute("transfer.recipient.amount", recipient_amount.to_string())
           .add_attribute("transfer.deduction.amount", deduction_amount.to_string())
       .add_attribute("transfer.recipient.member_deduction_amount", member_deduction_amount.to_string())
       .add_attribute("transfer.denom", denom)
       .add_attribute("transfer.sender", sender_addr)
       .add_attribute("transfer.recipient", recipient)
       .add_attribute("transfer.amount", amount.to_string())
       .add_attribute("transfer.block_time", env.block.time.seconds().to_string())
    )

}
