use serde::{Deserialize, Serialize};

use crate::archetype::ProblemArchetype;
use crate::constraints::ConstraintSet;
use crate::container::ContainerModel;
use crate::family::ProblemFamily;
use crate::item::ItemSpec;
use crate::mode::ProblemMode;
use crate::normalize::normalize_problem;
use crate::objective::Objective;
use crate::rules::PlacementRuleSet;
use crate::shape::{ShapeKind, ShapePair};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    pub family: ProblemFamily,
    #[serde(default)]
    pub archetype: Option<ProblemArchetype>,
    #[serde(default)]
    pub shape_pair: ShapePair,
    #[serde(default)]
    pub mode: ProblemMode,
    pub objective: Objective,
    pub items: ItemSpec,
    pub container: ContainerModel,
    #[serde(default)]
    pub rules: PlacementRuleSet,
    #[serde(default)]
    pub constraints: ConstraintSet,
}

impl Problem {
    pub fn circles_in_disk(radius: f64, container_r: f64, count: u32) -> Self {
        let mut p = Self {
            family: ProblemFamily::CircleSpherePacking,
            archetype: Some(ProblemArchetype::EqualCirclesInCircle),
            shape_pair: ShapePair::CIRCLES_IN_CIRCLE,
            mode: ProblemMode::EqualCopiesPacking,
            objective: Objective::MaxCount,
            items: ItemSpec::equal_circles(radius, count),
            container: ContainerModel::Circle { radius: container_r },
            rules: PlacementRuleSet::default(),
            constraints: ConstraintSet::default(),
        };
        normalize_problem(&mut p);
        p
    }

    pub fn circles_in_unit_square_variable(count: u32) -> Self {
        let mut p = Self {
            family: ProblemFamily::CircleSpherePacking,
            archetype: Some(ProblemArchetype::VariableCirclesMaxSum),
            shape_pair: ShapePair::CIRCLES_IN_SQUARE,
            mode: ProblemMode::EqualCopiesPacking,
            objective: Objective::MaxRadiusSum,
            items: ItemSpec::variable_circles(count),
            container: ContainerModel::unit_square(),
            rules: PlacementRuleSet::default(),
            constraints: ConstraintSet::default(),
        };
        normalize_problem(&mut p);
        p
    }

    /// AlphaEvolve 周长为 4 的矩形内最大化半径和。
    pub fn circles_in_perim_rect_variable(count: u32, perimeter: f64) -> Self {
        let mut p = Self {
            family: ProblemFamily::CircleSpherePacking,
            archetype: Some(ProblemArchetype::VariableCirclesInPerimRect),
            shape_pair: ShapePair::CIRCLES_IN_RECTANGLE,
            mode: ProblemMode::EqualCopiesPacking,
            objective: Objective::MaxRadiusSum,
            items: ItemSpec::variable_circles(count),
            container: ContainerModel::perim_rect_from_perimeter(perimeter, 1.0),
            rules: PlacementRuleSet::default(),
            constraints: ConstraintSet::default(),
        };
        normalize_problem(&mut p);
        p
    }

    pub fn cubes_in_cube(item_side: f64, container_side: f64, count: u32) -> Self {
        let mut p = Self {
            family: ProblemFamily::ManufacturerPalletLoading,
            archetype: Some(ProblemArchetype::CubesInCubes),
            shape_pair: ShapePair::CUBES_IN_CUBES,
            mode: ProblemMode::EqualCopiesPacking,
            objective: Objective::MaxCount,
            items: ItemSpec::orthogonal_cubes(item_side, count),
            container: ContainerModel::Cube {
                side: container_side,
            },
            rules: PlacementRuleSet::default(),
            constraints: ConstraintSet::default(),
        };
        normalize_problem(&mut p);
        p
    }

    pub fn one_dim_bin_packing(sizes: Vec<f64>) -> Self {
        let mut p = Self {
            family: ProblemFamily::ManufacturerPalletLoading,
            archetype: Some(ProblemArchetype::OneDimBinPacking),
            shape_pair: ShapePair {
                item: ShapeKind::Square,
                container: ShapeKind::Rectangle,
            },
            mode: ProblemMode::EqualCopiesPacking,
            objective: Objective::MinBins,
            items: ItemSpec::one_dim(sizes),
            container: ContainerModel::Rectangle {
                width: 1.0,
                height: 1.0,
            },
            rules: PlacementRuleSet::default(),
            constraints: ConstraintSet::default(),
        };
        normalize_problem(&mut p);
        p
    }

    pub fn normalized(mut self) -> Self {
        normalize_problem(&mut self);
        self
    }
}
