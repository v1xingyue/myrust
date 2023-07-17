use reqwest::{self, Error, Response};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct JsonResult<T> {
    pub jsonrpc: String,
    pub result: T,
}

#[derive(Serialize, Deserialize)]
pub struct SimpleObject {
    data: ObjectData,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectData {
    object_id: String,
    version: String,
    digest: String,
    #[serde(rename = "type")]
    object_type: String,
    owner: Owner,
    previous_transaction: String,
    #[serde(default)]
    storage_rebate: String,
    content: ObjectContent,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ObjectOwner")]
    object_owner: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectContent {
    data_type: String,
    #[serde(rename = "type")]
    object_type: String,
    has_public_transfer: bool,
    fields: Value,
}
