use crate::error::ContractError;
use crate::state::{
    read_config, read_pyth_feeder_config, store_config, store_pyth_feeder_config, Config,
    PythFeederConfig,
};
use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
use pyth_sdk_cw::PriceIdentifier;

/**
 * Update the config of the contract
 */
#[allow(clippy::too_many_arguments)]
pub fn config_feed_info(
    deps: DepsMut,
    info: MessageInfo,
    asset: String,
    price_feed_id: PriceIdentifier,
    price_feed_symbol: String,
    price_feed_decimal: u32,
    check_feed_age: bool,
    price_feed_age: u64,
) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let pyth_feeder_config = &PythFeederConfig {
        price_feed_id: price_feed_id.clone(),
        price_feed_symbol: price_feed_symbol.clone(),
        price_feed_decimal: price_feed_decimal.clone(),
        is_valid: true,
        check_feed_age: check_feed_age.clone(),
        price_feed_age: price_feed_age.clone(),
    };

    store_pyth_feeder_config(deps.storage, asset.clone(), &pyth_feeder_config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "config_feed_info"),
        ("asset_address", asset.as_str()),
        ("price_feed_id", &price_feed_id.to_string()),
        ("price_feed_symbol", &price_feed_symbol.clone()),
        ("price_feed_decimal", &price_feed_decimal.to_string()),
        ("check_feed_age", &check_feed_age.to_string()),
        ("price_feed_age", &price_feed_age.to_string()),
    ]))
}

/**
 * Update the config of the contract
 */
pub fn set_config_feed_valid(
    deps: DepsMut,
    info: MessageInfo,
    asset: String,
    is_valid: bool,
) -> Result<Response, ContractError> {
    let config = read_config(deps.storage)?;
    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    let mut pyth_feeder_config: PythFeederConfig =
        read_pyth_feeder_config(deps.storage, asset.clone())?;
    pyth_feeder_config.is_valid = is_valid;

    store_pyth_feeder_config(deps.storage, asset.clone(), &pyth_feeder_config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "set_config_feed_valid"),
        ("asset_address", asset.as_str()),
        ("is_valid", is_valid.to_string().as_str()),
    ]))
}

pub fn change_pyth_contract(
    deps: DepsMut,
    info: MessageInfo,
    new_pyth_contract: String,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    if deps.api.addr_canonicalize(info.sender.as_str())? != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.pyth_contract = deps
        .api
        .addr_canonicalize(&new_pyth_contract)
        .map_err(|_| ContractError::InvalidInput {})?;
    store_config(deps.storage, &config)?;

    Ok(Response::new().add_attributes(vec![
        ("action", "change_pyth_contract"),
        ("new_pyth_contract", new_pyth_contract.as_str()),
    ]))
}

pub fn set_owner(
    deps: DepsMut,
    info: MessageInfo,
    new_owner: Addr,
) -> Result<Response, ContractError> {
    let mut config: Config = read_config(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if sender_raw != config.owner {
        return Err(ContractError::Unauthorized {});
    }
    deps.api.addr_validate(new_owner.clone().as_str())?;

    config.new_owner = Some(deps.api.addr_canonicalize(new_owner.as_str())?);
    store_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "set_owner"),
        ("new_owner", new_owner.to_string().as_str()),
    ]))
}

pub fn accept_ownership(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = read_config(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    if config.new_owner.is_none() {
        return Err(ContractError::NoNewOwner {});
    }
    if sender_raw != config.new_owner.unwrap() {
        return Err(ContractError::Unauthorized {});
    }

    config.owner = sender_raw;
    config.new_owner = None;
    store_config(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        ("action", "accept_ownership"),
        ("owner", config.owner.clone().to_string().as_str()),
    ]))
}