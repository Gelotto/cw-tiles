use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Uint128, Uint64};

use crate::{
    error::ContractError,
    math::add_u64,
    state::{ADDR_2_ID, ID_2_ADDR, ID_COUNTER},
};

pub struct Coord((u32, u32, u8));

impl Coord {
    pub fn z(&self) {
        self.0
    }

    pub fn y(&self) {
        self.1
    }

    pub fn z(&self) {
        self.2
    }

    pub fn validate(
        &self,
        config: &Config,
    ) -> Result<(), ContractError> {
        if self.z() < config.min_zoom {
            return Err(ContractError::ValidationError {
                reason: format!("min zoom level is {config.min_zoom}"),
            });
        }
        if self.z() > config.max_zoom {
            return Err(ContractError::ValidationError {
                reason: format!("max zoom level is {config.max_zoom}"),
            });
        }
        let n_tiles_per_axis = (config.zoom_factor as u32).pow(self.z());
        if self.x() >= n_tiles_per_axis {
            return Err(ContractError::ValidationError {
                reason: "x coordinate out of bounds at given zoom level".to_owned(),
            });
        }
        if self.y() >= n_tiles_per_axis {
            return Err(ContractError::ValidationError {
                reason: "y coordinate out of bounds at given zoom level".to_owned(),
            });
        }
        Ok(())
    }
}

pub struct Id(Uint64);

impl Id {
    pub fn next_id(store: &mut dyn Storage) -> Result<Self, ContractError> {
        ID_COUNTER.update(store, |n| -> Result<_, ContractError> { add_u64(n, 1u64) })
    }
    pub fn get_addr_id(
        store: &mut dyn Storage,
        addr: &Addr,
    ) -> Result<Self, ContractError> {
        Ok(if let Some(id) = ADDR_2_ID.may_load(store, addr)? {
            id
        } else {
            let id = Self::next_id(store)?;
            ADDR_2_ID.save(store, addr, &id)?;
            ID_2_ADDR.save(store, id, addr)?;
            id
        })
    }
}

#[cw_serde]
pub struct Config {
    pub max_zoom: u8,
    pub min_zoom: u8,
    pub zoom_factor: u8,
    pub unit_price: Uint128,
    pub price_discount_pct: Uint128,
}

#[cw_serde]
pub struct TileInfo {
    pub owner_id: u32,
}
