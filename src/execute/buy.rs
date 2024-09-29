use crate::{
    error::ContractError,
    math::{mul_pct_u128, mul_u128},
    models::{Config, Coord},
    msg::BuyMsg,
    state::{models::Config, CONFIG},
};
use cosmwasm_std::{attr, Response, Uint128};

use super::Context;

pub fn exec_buy(
    ctx: Context,
    msg: BuyMsg,
) -> Result<Response, ContractError> {
    let Context { deps, .. } = ctx;
    let BuyMsg { coord } = msg;
    let config = CONFIG.load(store)?;

    coord.validate(config)?;

    let price = calc_tile_price(coord, &config)?;

    Ok(Response::new().add_attributes(vec![attr("action", "buy")]))
}

fn calc_tile_price(
    coord: Coord,
    config: &Config,
) -> Uint128 {
    let Config {
        max_zoom,
        price_discount_pct,
        unit_price,
        zoom_factor,
        ..
    } = config;
    let zoom_level = coord.z() as u32;
    let multiplier = zoom_factor.pow(max_zoom - zoom_level);
    let full_price = mul_u128(unit_price, multiplier as u128)?;
    let discounted_price = mul_pct_u128(full_price, price_discount_pct)?;
    Ok(discounted_price)
}
