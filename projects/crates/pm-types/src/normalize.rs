use crate::archetype::ProblemArchetype;
use crate::family::ProblemFamily;
use crate::mode::ProblemMode;
use crate::objective::Objective;
use crate::problem::Problem;
use crate::shape::{ShapeKind, ShapePair};

pub fn normalize_problem(problem: &mut Problem) {
    if problem.archetype.is_none() {
        problem.archetype = infer_archetype(problem);
    }
    if let Some(arch) = problem.archetype {
        if problem.shape_pair == ShapePair::default() {
            problem.shape_pair = shape_pair_from_archetype(arch);
        }
        if arch == ProblemArchetype::MaxPerimeterCirclesInSquare {
            problem.mode = ProblemMode::MaxTotalPerimeter;
        } else if matches!(
            arch,
            ProblemArchetype::TammesPointsInDisk | ProblemArchetype::CongruentCirclesCountBound
        ) {
            problem.mode = ProblemMode::Dispersion;
        }
        problem.family = family_from_archetype(arch);
    }
    if problem.archetype.is_none() {
        problem.archetype = infer_archetype(problem);
    }
}

fn family_from_archetype(archetype: ProblemArchetype) -> ProblemFamily {
    match archetype {
        ProblemArchetype::OneDimBinPacking
        | ProblemArchetype::TwoDimStripPacking
        | ProblemArchetype::ThreeDimBinPacking
        | ProblemArchetype::CubesInCubes
        | ProblemArchetype::TetrahedraInCubes
        | ProblemArchetype::OctahedraInCubes => ProblemFamily::ManufacturerPalletLoading,
        _ => ProblemFamily::CircleSpherePacking,
    }
}

pub fn infer_archetype(problem: &Problem) -> Option<ProblemArchetype> {
    match (problem.shape_pair, problem.mode, problem.objective) {
        (ShapePair::CIRCLES_IN_CIRCLE, ProblemMode::Dispersion, _) => {
            Some(ProblemArchetype::TammesPointsInDisk)
        }
        (ShapePair::CIRCLES_IN_CIRCLE, _, Objective::MaxCount) if matches!(
            problem.items.model,
            crate::item::ItemModel::Circle { .. }
                | crate::item::ItemModel::IdenticalCircles { .. }
        ) && matches!(problem.container, crate::container::ContainerModel::Circle { .. }) => {
            Some(ProblemArchetype::CongruentCirclesCountBound)
        }
        (ShapePair::CIRCLES_IN_CIRCLE, _, _) => Some(ProblemArchetype::EqualCirclesInCircle),
        (ShapePair::CIRCLES_IN_SQUARE, _, Objective::MaxRadiusSum) => {
            Some(ProblemArchetype::VariableCirclesMaxSum)
        }
        (ShapePair::CIRCLES_IN_SQUARE, ProblemMode::MaxTotalPerimeter, _) => {
            Some(ProblemArchetype::MaxPerimeterCirclesInSquare)
        }
        (ShapePair::CIRCLES_IN_SQUARE, _, _) => Some(ProblemArchetype::EqualCirclesInSquare),
        (ShapePair::CIRCLES_IN_RECTANGLE, _, Objective::MaxRadiusSum) => {
            Some(ProblemArchetype::VariableCirclesInPerimRect)
        }
        (ShapePair::CIRCLES_IN_RECTANGLE, _, _) => Some(ProblemArchetype::EqualCirclesInRectangle),
        (ShapePair::CUBES_IN_CUBES, _, _) => Some(ProblemArchetype::CubesInCubes),
        _ if problem.family == ProblemFamily::ManufacturerPalletLoading => {
            if matches!(problem.items.model, crate::item::ItemModel::OrthogonalBoxes { .. }) {
                Some(ProblemArchetype::TwoDimStripPacking)
            } else if matches!(problem.items.model, crate::item::ItemModel::OneDimItems { .. }) {
                Some(ProblemArchetype::OneDimBinPacking)
            } else {
                Some(ProblemArchetype::ThreeDimBinPacking)
            }
        }
        _ => None,
    }
}

pub fn shape_pair_from_archetype(archetype: ProblemArchetype) -> ShapePair {
    match archetype {
        ProblemArchetype::EqualCirclesInCircle
        | ProblemArchetype::TammesPointsInDisk
        | ProblemArchetype::CongruentCirclesCountBound => ShapePair::CIRCLES_IN_CIRCLE,
        ProblemArchetype::EqualCirclesInSquare
        | ProblemArchetype::VariableCirclesMaxSum
        | ProblemArchetype::MaxPerimeterCirclesInSquare => ShapePair::CIRCLES_IN_SQUARE,
        ProblemArchetype::EqualCirclesInRectangle
        | ProblemArchetype::VariableCirclesInPerimRect => ShapePair::CIRCLES_IN_RECTANGLE,
        ProblemArchetype::CubesInCubes => ShapePair::CUBES_IN_CUBES,
        ProblemArchetype::TetrahedraInCubes => ShapePair {
            item: ShapeKind::Tetrahedron,
            container: ShapeKind::Cube,
        },
        ProblemArchetype::OctahedraInCubes => ShapePair {
            item: ShapeKind::Octahedron,
            container: ShapeKind::Cube,
        },
        ProblemArchetype::TwoDimStripPacking => ShapePair {
            item: ShapeKind::Square,
            container: ShapeKind::Rectangle,
        },
        ProblemArchetype::OneDimBinPacking
        | ProblemArchetype::ThreeDimBinPacking
        | ProblemArchetype::EqualCirclesInEllipse => ShapePair {
            item: ShapeKind::OrthogonalBox,
            container: ShapeKind::OrthogonalBox,
        },
    }
}

pub fn archetype_from_shape_mode(
    pair: ShapePair,
    mode: ProblemMode,
    objective: Objective,
) -> Option<ProblemArchetype> {
    let p = Problem {
        family: ProblemFamily::CircleSpherePacking,
        archetype: None,
        shape_pair: pair,
        mode,
        objective,
        items: crate::item::ItemSpec::equal_circles(1.0, 1),
        container: crate::container::ContainerModel::Circle { radius: 1.0 },
        rules: crate::rules::PlacementRuleSet::default(),
        constraints: crate::constraints::ConstraintSet::default(),
    };
    infer_archetype(&p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objective::Objective;
    use crate::shape::ShapePair;

    #[test]
    fn normalize_circles_in_square() {
        let arch = archetype_from_shape_mode(
            ShapePair::CIRCLES_IN_SQUARE,
            ProblemMode::EqualCopiesPacking,
            Objective::MaxEqualRadius,
        );
        assert_eq!(arch, Some(ProblemArchetype::EqualCirclesInSquare));
    }

    #[test]
    fn variable_circles_archetype() {
        let mut p = Problem::circles_in_unit_square_variable(26);
        normalize_problem(&mut p);
        assert_eq!(p.archetype, Some(ProblemArchetype::VariableCirclesMaxSum));
    }
}
