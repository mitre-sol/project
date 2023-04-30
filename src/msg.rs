use crate::state::Config;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub struct GetOwner {}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer { address1: Addr, address2: Addr, amount: Uint128 },
    Withdraw { amount: Uint128 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(GetOwnerResponse)]
    GetOwner {},
    #[returns(GetBalanceResponse)]
    GetBalance { address: Addr },
}

#[cw_serde]
pub struct ConfigResponse {
    owner: Addr,
}

impl From<Config> for ConfigResponse {
    fn from(config: Config) -> ConfigResponse {
        ConfigResponse {
            owner: config.owner,
        }
    }
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub address: String,
}

#[cw_serde]
pub struct GetBalanceResponse {
    pub balance: Uint128,
}
