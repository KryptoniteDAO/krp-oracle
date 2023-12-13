use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, StdError, StdResult, Storage};

use cw_storage_plus::{Item, Map};
use pyth_sdk_cw::PriceIdentifier;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PythFeederConfig {
    pub price_feed_id: PriceIdentifier,
    pub price_feed_symbol: String,
    pub price_feed_decimal: u32,
    pub is_valid: bool,
    pub check_feed_age: bool,
    pub price_feed_age: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub pyth_contract: CanonicalAddr,
    pub new_owner: Option<CanonicalAddr>,
}

pub const PYTH_FEEDER_CONFIG: Map<String, PythFeederConfig> = Map::new("pyth_feeder_config");

pub const KEY_CONFIG: Item<Config> = Item::new("config");


pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    KEY_CONFIG.save(storage, config)
}

pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    KEY_CONFIG.load(storage)
}


pub fn store_pyth_feeder_config(
    storage: &mut dyn Storage,
    asset: String,
    pyth_feeder_config: &PythFeederConfig,
) -> Result<PythFeederConfig, StdError> {
    PYTH_FEEDER_CONFIG.update(storage, asset, |old| match old {
        Some(_) => Ok(pyth_feeder_config.clone()),
        None => Ok(pyth_feeder_config.clone()),
    })
}

pub fn read_pyth_feeder_config(
    storage: &dyn Storage,
    asset: String,
) -> Result<PythFeederConfig, StdError> {
    let pyth_feeder_config = PYTH_FEEDER_CONFIG
        .may_load(storage, asset)?
        .ok_or_else(|| StdError::generic_err("Pyth feeder config not found"));
    Ok(pyth_feeder_config.unwrap())
}
