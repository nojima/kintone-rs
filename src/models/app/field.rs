#[derive(Debug, Clone, PartialEq, Eq)]
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
