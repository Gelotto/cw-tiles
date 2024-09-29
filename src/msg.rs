use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::{models::Coord, responses::ConfigResponse, state::models::Config};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
#[derive(cw_orch::ExecuteFns)]
pub enum ExecuteMsg {
    SetConfig(Config),
    Buy(BuyMsg),
}

#[cw_serde]
#[derive(cw_orch::QueryFns, QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct BuyMsg {
    pub coord: Coord,
}
