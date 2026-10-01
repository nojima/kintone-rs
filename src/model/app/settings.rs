//! Models for app settings, process management, notifications and permissions.

use serde::{Deserialize, Serialize};

use super::field::RelatedApp;
use crate::internal::serde_helper::{option_stringified, stringified};
use crate::model::{Entity, FileBody};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SettingsEntityType {
    #[default]
    User,
    Group,
    Organization,
    FieldEntity,
    Creator,
    CustomField,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsEntity {
    #[serde(rename = "type")]
    pub entity_type: SettingsEntityType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityTarget {
    pub entity: SettingsEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Theme {
    #[default]
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Black,
    Clipboard,
    Binder,
    Pencil,
    Clips,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SelectionMode {
    #[default]
    Auto,
    Manual,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RoundingMode {
    #[default]
    HalfEven,
    Up,
    Down,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TitleField {
    pub selection_mode: SelectionMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberPrecision {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub digits: Option<u64>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub decimal_places: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rounding_mode: Option<RoundingMode>,
}

/// App icons accept either a built in key or an uploaded file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppIcon {
    Preset { key: String },
    File { file: FileBody },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssigneeType {
    #[default]
    One,
    All,
    Any,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessAssignee {
    #[serde(rename = "type")]
    pub assignee_type: AssigneeType,
    pub entities: Vec<EntityTarget>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(with = "stringified")]
    pub index: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assignee: Option<ProcessAssignee>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProcessActionType {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutableUser {
    pub entities: Vec<EntityTarget>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessAction {
    pub name: String,
    pub from: String,
    pub to: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub action_type: Option<ProcessActionType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable_user: Option<ExecutableUser>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomizationScope {
    #[default]
    All,
    Admin,
    None,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Customization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub js: Option<Vec<CustomizationResource>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub css: Option<Vec<CustomizationResource>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomizationResource {
    Url { url: String },
    File { file: FileBody },
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneralNotification {
    pub entity: SettingsEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_added: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_edited: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_added: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_changed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_imported: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerRecordNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub targets: Vec<EntityTarget>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderTiming {
    pub code: String,
    #[serde(with = "stringified")]
    pub days_later: i64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub hours_later: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderNotification {
    pub timing: ReminderTiming,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub targets: Vec<EntityTarget>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppRight {
    pub entity: SettingsEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub app_editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_viewable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_addable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_deletable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_importable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub record_exportable: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordRight {
    pub filter_cond: String,
    pub entities: Vec<RecordRightEntity>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordRightEntity {
    pub entity: SettingsEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Accessibility {
    #[default]
    Read,
    Write,
    None,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRight {
    pub code: String,
    pub entities: Vec<FieldRightEntity>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldRightEntity {
    pub accessibility: Accessibility,
    pub entity: SettingsEntity,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_subs: Option<bool>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActionSourceType {
    #[default]
    Field,
    RecordUrl,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionMapping {
    pub src_type: ActionSourceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src_field: Option<String>,
    pub dest_field: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified",
        skip_serializing
    )]
    pub id: Option<u64>,
    #[serde(with = "stringified")]
    pub index: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dest_app: Option<RelatedApp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<ActionMapping>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppPlugin {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}
