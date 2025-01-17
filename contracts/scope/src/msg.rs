use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;
use provwasm_std::Scope;

#[cw_serde]
pub struct InitMsg {
    pub name: String, // Bind this name to the contract address (eg contract.pb).
}

#[cw_serde]
pub enum ExecuteMsg {
    WriteScope { scope: Scope, signers: Vec<Addr> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(provwasm_std::Scope)]
    GetScope { id: String },
    #[returns(provwasm_std::Sessions)]
    GetSessions { scope_id: String },
    #[returns(provwasm_std::Records)]
    GetRecords { scope_id: String },
    #[returns(provwasm_std::Record)]
    GetRecordByName { scope_id: String, name: String },
}
