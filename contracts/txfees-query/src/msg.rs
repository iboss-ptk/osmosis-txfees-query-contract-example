use cosmwasm_schema::{cw_serde, QueryResponses};
use osmosis_std::types::osmosis::txfees::v1beta1::{
    QueryDenomPoolIdResponse, QueryFeeTokensResponse,
};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    #[returns(QueryDenomPoolIdResponse)]
    QueryDenomPoolId { denom: String },

    #[returns(QueryFeeTokensResponse)]
    QueryFeeTokens {},
}
