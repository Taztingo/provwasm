use cosmwasm_std::{Binary, Coin, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Supported provenance module router keys.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceRoute {
    Attribute,
    Name,
    Marker,
    Metadata,
}

/// A collection of bound names.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Names {
    pub records: Vec<Name>,
}

/// A name bound to an address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Name {
    pub name: String,
    pub address: HumanAddr,
    pub restricted: bool,
}

/// A collection of attributes associated with an account address.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attributes {
    pub address: HumanAddr,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

/// Allowed attribute value types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AttributeValueType {
    Uuid,
    Json,
    String,
    Bytes,
}

/// A typed key-value pair.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    pub name: String,
    pub value: Binary,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub value_type: AttributeValueType,
}

/// Metadata from the provenance execution environment.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Scope {
    pub id: String,
    pub parties: Vec<Party>,
    pub record_groups: Vec<RecordGroup>,
}

/// A party on a scope.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Party {
    pub address: String,
    pub role: i64,
}

/// A group of records on a scope.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RecordGroup {
    pub class_name: String,
    pub group_id: String,
    pub parties: Vec<Party>,
    pub records: Vec<Record>,
    pub specification: String,
}

/// A record in a record group.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Record {
    pub class_name: String,
    pub hash: String,
    pub inputs: Vec<RecordInput>,
    pub name: String,
    pub result_hash: String,
    pub result_name: String,
    pub result_type: i64,
}

/// An input that produced a record.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct RecordInput {
    pub class_name: String,
    pub hash: String,
    pub input_type: i64,
    pub name: String,
}

// A marker account
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Marker {
    pub address: HumanAddr,
    #[serde(default)]
    pub coins: Vec<Coin>,
    #[serde(default)]
    pub public_key: Binary,
    pub account_number: u64,
    pub sequence: u64,
    #[serde(default)]
    pub manager: HumanAddr,
    #[serde(default)]
    pub permissions: Vec<AccessGrant>,
    pub status: String,
    pub denom: String,
    pub total_supply: String,
    pub marker_type: String,
}

// Marker permissions granted to another account.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessGrant {
    pub permissions: Vec<MarkerPermission>,
    pub address: HumanAddr,
}

/// Marker permission types.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MarkerPermission {
    Burn,
    Deposit,
    Delete,
    Grant,
    Mint,
    Withdraw,
}
