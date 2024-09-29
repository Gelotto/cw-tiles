use cosmwasm_schema::{cw_serde, QueryResponses};

use crate::state::models::Config;

#[cw_serde]
pub struct ConfigResponse(pub Config);
