use serde::{Deserialize, Serialize};

use crate::family::ProblemFamily;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Placement2d {
    pub id: u32,
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolutionMetrics {
    pub count: u32,
    pub density: f64,
    pub container_area: f64,
    pub packed_area: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SolveMeta {
    pub algorithm: String,
    pub backend: String,
    pub iterations: u32,
    pub elapsed_ms: u64,
}

impl Default for SolveMeta {
    fn default() -> Self {
        Self {
            algorithm: String::new(),
            backend: String::new(),
            iterations: 0,
            elapsed_ms: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Solution {
    pub family: ProblemFamily,
    pub placements: Vec<Placement2d>,
    pub metrics: SolutionMetrics,
    pub feasible: bool,
    pub meta: SolveMeta,
}
