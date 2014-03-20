//! Packing-Man 问题分类与求解合同。

mod container;
mod error;
mod family;
mod item;
mod objective;
mod problem;
mod rules;
mod solution;
mod solver;

pub use container::ContainerModel;
pub use error::{PmError, PmResult};
pub use family::ProblemFamily;
pub use item::{ItemModel, ItemSpec};
pub use objective::Objective;
pub use problem::Problem;
pub use rules::{PlacementRule, PlacementRuleSet};
pub use solution::{Placement2d, Solution, SolutionMetrics, SolveMeta};
pub use solver::{AlgorithmKind, Backend, SolverId};
