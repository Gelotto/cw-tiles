use cosmwasm_std::{Addr, Response, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use crate::{
    error::ContractError,
    execute::Context,
    models::{Coord, TileInfo},
    msg::InstantiateMsg,
};

pub const CONFIG: Item<Config> = Item::new("config");
pub const TILES: Map<Coord, TileInfo> = Map::new("tiles");
pub const ID_COUNTER: Item<Uint64> = Item::new("idc");
pub const ID_2_OWNED_COORD: Map<Id, Coord> = Map::new("id_2_owned_coord");
pub const ID_2_ADDR: Map<Id, Addr> = Map::new("id_2_addr");
pub const ADDR_2_ID: Map<&Addr, Id> = Map::new("addr_2_id");

/// Top-level initialization of contract state
pub fn init(
    _ctx: Context,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::new().add_attribute("action", "instantiate"))
}
