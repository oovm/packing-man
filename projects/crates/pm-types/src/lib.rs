//! Packing-Man 问题分类与求解合同。

mod archetype;
mod constraints;
mod container;
mod error;
mod family;
mod item;
mod mode;
mod normalize;
mod objective;
mod problem;
mod rules;
mod shape;
mod solution;
mod solver;

pub use archetype::ProblemArchetype;
pub use constraints::{ArrivalMode, AxleLoad, ConstraintSet, StackRules};
pub use container::{ContainerModel, VehicleSpec};
pub use error::{PmError, PmResult};
pub use family::ProblemFamily;
pub use item::{BoxItemKind, BoxSpec, ItemModel, ItemSpec};
pub use mode::ProblemMode;
pub use normalize::{archetype_from_shape_mode, infer_archetype, normalize_problem};
pub use objective::Objective;
pub use problem::Problem;
pub use rules::{PlacementRule, PlacementRuleSet};
pub use shape::{PolyominoKind, ShapeKind, ShapePair};
pub use solution::{
    BinPlacement1d, BinSolution, Placement2d, Placement3d, PlacementRect, Solution, SolutionMetrics,
    SolveMeta,
};
pub use solver::{AlgorithmKind, Backend, SolverId};
