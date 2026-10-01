//! Installed plugins and the apps that use them.

use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::stringified;

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_market_plugin: bool,
    pub version: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequiredPlugin {
    pub id: String,
    pub name: String,
    pub is_market_plugin: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginApp {
    #[serde(with = "stringified")]
    pub id: u64,
    pub name: String,
}
