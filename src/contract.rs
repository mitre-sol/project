use cosmwasm_std::{
    Addr, entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128
};

use crate::error::ContractError;
use crate::msg::{ConfigResponse, ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse};
use crate::state::{Config, NameRecord, CONFIG, BALANCES};

const MIN_NAME_LENGTH: u64 = 3;
const MAX_NAME_LENGTH: u64 = 64;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let config = Config {
        owner: info.sender.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    println!("INFO {}", info.sender.into_string());

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { address1, address2, amount } => execute_transfer(deps, env, info, address1, address2, amount),
    }
}

// This utility was copied directly from the name-service example.
pub fn assert_sent_sufficient_coin(
    sent: &[Coin],
    required: Option<Coin>,
) -> Result<(), ContractError> {
    if let Some(required_coin) = required {
        let required_amount = required_coin.amount.u128();
        if required_amount > 0 {
            let sent_sufficient_funds = sent.iter().any(|coin| {
                // check if a given sent coin matches denom
                // and has sufficient amount
                coin.denom == required_coin.denom && coin.amount.u128() >= required_amount
            });

            if sent_sufficient_funds {
                return Ok(());
            } else {
                return Err(ContractError::InsufficientFundsSend {});
            }
        }
    }
    Ok(())
}

pub fn execute_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    address1: Addr,
    address2: Addr,
    amount: Uint128,
) -> Result<Response, ContractError> {
    // Check that requestor has sufficient usei tokens.
    assert_sent_sufficient_coin(&info.funds, Some(Coin{ denom: "usei".to_string(), amount: amount}))?;

    // lambda function taken from cw-storage-plus tests:  https://github.com/CosmWasm/cw-storage-plus/blob/main/src/map.rs#LL1181C1-L1182C1
    let add_half_amount = |a: Option<Uint128>| -> StdResult<_> { Ok(a.unwrap_or_default().checked_add(amount.checked_div(2u128.into()).unwrap()).unwrap()) };

    BALANCES.update(deps.storage, address1, add_half_amount);
    BALANCES.update(deps.storage, address2, add_half_amount);

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary::<ConfigResponse>(&CONFIG.load(deps.storage)?.into()),
        QueryMsg::GetOwner {} => get_owner_resolver(deps, env),
    }
}

fn get_owner_resolver(deps: Deps, _env: Env) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    let address = config.owner.to_string();
    let resp = GetOwnerResponse { address };
    to_binary(&resp)
}
