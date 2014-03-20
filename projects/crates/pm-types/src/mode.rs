use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProblemMode {
    #[default]
    EqualCopiesPacking,
    MaxTotalPerimeter,
    Covering,
    Dispersion,
    Consecutive,
    Tiling,
}
