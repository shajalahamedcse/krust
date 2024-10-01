use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[kube(group = "example.com", version = "v1", kind = "Pinger", namespaced)]
pub struct PingerSpec {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct PingerStatus {
    pub last_ping: Option<String>,
}