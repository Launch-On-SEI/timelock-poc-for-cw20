use cosmwasm_std::{
    entry_point, to_binary, BankMsg, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo,
    Response, StdResult, Uint128, WasmMsg, StdError,
};
use cw20::Cw20ReceiveMsg;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: String,
    pub deposit: Uint128,
    pub unlock_times: Vec<u64>,
    pub recipients: [String; 4],
    pub allocation: [Uint128; 4],
    pub last_disburse: u64,
}

const STATE: Item<State> = Item::new("state");
const DISBURSEMENT_TRACKER: Map<u64, bool> = Map::new("disbursement_tracker");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        owner: "owner_address".to_string(),
        deposit: Uint128::zero(),
        unlock_times: vec![
            _env.block.time.seconds() + 6 * 30 * 24 * 60 * 60, // 6 months in seconds
            _env.block.time.seconds() + 12 * 30 * 24 * 60 * 60, // 12 months
            _env.block.time.seconds() + 18 * 30 * 24 * 60 * 60, // 18 months
            _env.block.time.seconds() + 24 * 30 * 24 * 60 * 60, // 24 months
        ],
        recipients: [
            "recipient1".to_string(),
            "recipient2".to_string(),
            "recipient3".to_string(),
            "recipient4".to_string(),
        ],
        allocation: [
            Uint128::zero(),
            Uint128::zero(),
            Uint128::zero(),
            Uint128::zero(),
        ],
        last_disburse: 0,
    };

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Receive(msg) => try_receive(deps, env, info, msg),
        ExecuteMsg::Disburse {} => try_disburse(deps, env, info),
    }
}

pub fn try_receive(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: Cw20ReceiveMsg,
) -> StdResult<Response> {
    let sender = info.sender;
    let amount = msg.amount;
    let mut state = STATE.load(deps.storage)?;

    // Ensure only the contract owner can deposit
    if sender != deps.api.addr_validate(&state.owner)? {
        return Err(StdError::generic_err("Unauthorized"));
    }

    // Update deposit amount
    state.deposit += amount;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "deposit").add_attribute("amount", amount.to_string()))
}

pub fn try_disburse(deps: DepsMut, env: Env, _info: MessageInfo) -> StdResult<Response> {
    let state = STATE.load(deps.storage)?;
    let mut messages: Vec<CosmosMsg> = vec![];
    let current_time = env.block.time.seconds();

    for (index, &unlock_time) in state.unlock_times.iter().enumerate() {
        if current_time >= unlock_time && state.last_disburse < unlock_time {
            // Check if this disbursement period has already been redeemed
            if DISBURSEMENT_TRACKER.may_load(deps.storage, unlock_time)?.is_none() {
                let amount_per_recipient = state.deposit.u128() / 4 / state.unlock_times.len() as u128; // Divide equally

                for recipient in state.recipients.iter() {
                    messages.push(CosmosMsg::Bank(BankMsg::Send {
                        to_address: recipient.to_string(),
                        amount: vec![Coin {
                            denom: "token".to_string(),
                            amount: Uint128::from(amount_per_recipient),
                        }],
                    }));
                }

                // Mark this disbursement as redeemed
                DISBURSEMENT_TRACKER.save(deps.storage, unlock_time, &true)?;
            }
        }
    }

    // Update last disburse time if disbursements were made
    if !messages.is_empty() {
        STATE.update(deps.storage, |mut state| -> StdResult<_> {
            state.last_disburse = current_time;
            Ok(state)
        })?;
    }

    Ok(Response::new()
        .add_messages(messages)
        .add_attribute("action", "disburse"))
}

#[entry_point]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    // Implement query logic if needed
    unimplemented!();
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive(Cw20ReceiveMsg),
    Disburse {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // Define query messages if needed
}
