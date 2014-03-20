use serde::{Deserialize, Serialize};

use crate::family::ProblemFamily;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Placement2d {
    pub id: u32,
    pub cx: f64,
    pub cy: f64,
    pub radius: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlacementRect {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Placement3d {
    pub id: u32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
    pub bin_id: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinPlacement1d {
    pub bin_id: u32,
    pub item_indices: Vec<usize>,
    pub used: f64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SolutionMetrics {
    pub count: u32,
    pub density: f64,
    pub container_area: f64,
    pub packed_area: f64,
    #[serde(default)]
    pub radius_sum: f64,
    #[serde(default)]
    pub min_pairwise_distance: f64,
    #[serde(default)]
    pub equal_radius: f64,
    #[serde(default)]
    pub bin_count: u32,
    #[serde(default)]
    pub volume_utilization: f64,
    #[serde(default)]
    pub objective_value: f64,
    #[serde(default)]
    pub bound_gap: f64,
    #[serde(default)]
    pub strip_height: f64,
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
pub struct BinSolution {
    pub bin_id: u32,
    pub placements_3d: Vec<Placement3d>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Solution {
    pub family: ProblemFamily,
    pub placements: Vec<Placement2d>,
    #[serde(default)]
    pub rect_placements: Vec<PlacementRect>,
    #[serde(default)]
    pub placements_3d: Vec<Placement3d>,
    #[serde(default)]
    pub bins_1d: Vec<BinPlacement1d>,
    #[serde(default)]
    pub bins: Vec<BinSolution>,
    pub metrics: SolutionMetrics,
    pub feasible: bool,
    pub meta: SolveMeta,
}
