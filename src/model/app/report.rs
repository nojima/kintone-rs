//! Graphs and periodic report settings.

use serde::{Deserialize, Serialize};

use crate::internal::serde_helper::{option_stringified, stringified};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartType {
    #[default]
    Bar,
    Column,
    Pie,
    Line,
    PivotTable,
    Table,
    Area,
    Spline,
    SplineArea,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChartMode {
    #[default]
    Normal,
    Stacked,
    Percentage,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeUnit {
    #[default]
    Year,
    Quarter,
    Month,
    Week,
    Day,
    Hour,
    Minute,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AggregationType {
    #[default]
    Count,
    Sum,
    Average,
    Max,
    Min,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportSortBy {
    #[default]
    Total,
    Group1,
    Group2,
    Group3,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportSortOrder {
    #[default]
    Asc,
    Desc,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportInterval {
    #[default]
    Year,
    Quarter,
    Month,
    Week,
    Day,
    Hour,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuarterPattern {
    #[default]
    JanAprJulOct,
    FebMayAugNov,
    MarJunSepDec,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayOfWeek {
    #[default]
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Graph {
    pub chart_type: ChartType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chart_mode: Option<ChartMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(with = "stringified")]
    pub index: u64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified",
        skip_serializing
    )]
    pub id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<GraphGroup>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<Vec<GraphAggregation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_cond: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sorts: Option<Vec<GraphSort>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periodic_report: Option<PeriodicReport>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphGroup {
    pub code: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub per: Option<TimeUnit>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphAggregation {
    #[serde(rename = "type")]
    pub aggregation_type: AggregationType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphSort {
    pub by: ReportSortBy,
    pub order: ReportSortOrder,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PeriodicReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<ReportPeriod>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportPeriod {
    pub every: ReportInterval,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub month: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<QuarterPattern>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_of_month: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub day_of_week: Option<DayOfWeek>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_stringified"
    )]
    pub minute: Option<u64>,
}
