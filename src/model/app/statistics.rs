//! App usage statistics (wide course).

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::stringified;
use crate::model::User;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppStatus {
    #[default]
    Changed,
    NotActivated,
    Activated,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSpace {
    #[serde(with = "stringified")]
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStatistic {
    #[serde(with = "stringified")]
    pub id: u64,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space: Option<AppSpace>,
    pub app_group: String,
    pub status: AppStatus,
    pub record_updated_at: DateTime<FixedOffset>,
    #[serde(with = "stringified")]
    pub record_count: u64,
    #[serde(with = "stringified")]
    pub field_count: u64,
    #[serde(with = "stringified")]
    pub daily_request_count: u64,
    #[serde(with = "stringified")]
    pub api_token_count: u64,
    #[serde(with = "stringified")]
    pub webhook_count: u64,
    #[serde(with = "stringified")]
    pub storage_usage: u64,
    pub customized: bool,
    pub creator: User,
    pub created_at: DateTime<FixedOffset>,
    pub modifier: User,
    pub modified_at: DateTime<FixedOffset>,
}
