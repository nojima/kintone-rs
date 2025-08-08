use enum_assoc::Assoc;
use serde::{Deserialize, Serialize};

use crate::models::record::FieldType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Assoc)]
#[serde(tag = "type", content = "value", rename_all = "SCREAMING_SNAKE_CASE")]
#[func(pub const fn field_type(&self) -> FieldType)]
#[non_exhaustive]
pub enum FieldProperty {
    #[assoc(field_type = FieldType::Calc)]
    Calc(CalcFieldProperty),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalcFieldProperty {
    pub code: String,
    pub label: String,
    pub no_label: bool,
    pub required: bool,
    pub expression: String,
    //pub format: DisplayFormat,
    pub display_scale: i64,
    pub hide_expression: bool,
    pub unit: String,
    //pub unit_position: UnitPosition,
}
