use crate::framework::response::ApiResult;

mod create_custom_hostname;
mod delete_custom_hostname;
mod edit_custom_hostname;

pub use create_custom_hostname::*;
pub use delete_custom_hostname::*;
pub use edit_custom_hostname::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivationStatus {
    Active,
    Pending,
    ActiveRedeploying,
    Moved,
    PendingDeletion,
    Deleted,
    PendingBlocked,
    PendingMigration,
    PendingProvisioned,
    TestPending,
    TestActive,
    TestActiveApex,
    TestBlocked,
    TestFailed,
    Provisioned,
    Blocked,
}

// https://api.cloudflare.com/#custom-hostname-for-a-zone-properties
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct CustomHostame {
    pub id: String,
    pub status: ActivationStatus,
}

impl ApiResult for CustomHostame {}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SslParams {
    pub method: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub settings: SslSettingsParams,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wildcard: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_certificate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_key: Option<String>,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct SslSettingsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http2: Option<String>,
    pub min_tls_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tls_1_3: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ciphers: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_hints: Option<String>,
}
