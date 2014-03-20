use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementRule {
    Orthogonal,
    Rot90,
    FreeRotation,
    Guillotine,
    NonGuillotine,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct PlacementRuleSet {
    pub rules: Vec<PlacementRule>,
}
