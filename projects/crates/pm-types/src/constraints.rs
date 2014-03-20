use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ArrivalMode {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StackRules {
    #[serde(default)]
    pub fragile_no_stack: bool,
    #[serde(default)]
    pub max_pressure_kg_m2: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AxleLoad {
    pub front_max_kg: f64,
    pub rear_max_kg: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ConstraintSet {
    #[serde(default)]
    pub arrival: ArrivalMode,
    #[serde(default)]
    pub stack_rules: StackRules,
    #[serde(default)]
    pub axle_load: Option<AxleLoad>,
    #[serde(default)]
    pub top_clearance_cm: Option<f64>,
}
