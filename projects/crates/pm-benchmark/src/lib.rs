//! 按 `ProblemFamily` / `ProblemArchetype` 分类的基准实例。

mod registry;

use pm_types::{
    normalize_problem, Objective, Problem, ProblemArchetype, Solution, SolverId,
};

pub use registry::{all_paths, get as fixture_json};

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct FixtureExpect {
    #[serde(default)]
    pub objective_record: Option<f64>,
    #[serde(default)]
    pub bin_count: Option<u32>,
    #[serde(default)]
    pub optimality: Option<String>,
    #[serde(default)]
    pub feasible: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Fixture {
    pub id: String,
    pub problem: Problem,
    #[serde(default)]
    pub expect: FixtureExpect,
}

pub fn load_fixture(json: &str) -> Result<Fixture, serde_json::Error> {
    serde_json::from_str(json)
}

pub fn load_by_path(path: &str) -> Option<Fixture> {
    let json = registry::get(path)?;
    let mut fixture = load_fixture(json).ok()?;
    normalize_problem(&mut fixture.problem);
    Some(fixture)
}

pub fn problem_from_fixture(fixture: &Fixture) -> Problem {
    let mut p = fixture.problem.clone();
    normalize_problem(&mut p);
    p
}

pub fn solver_for_problem(problem: &Problem) -> SolverId {
    match problem.archetype {
        Some(ProblemArchetype::OneDimBinPacking) => SolverId::CPU_FFD,
        Some(ProblemArchetype::CubesInCubes) | Some(ProblemArchetype::ThreeDimBinPacking) => {
            SolverId::CPU_EXTREME_POINT
        }
        Some(ProblemArchetype::TwoDimStripPacking) => SolverId::CPU_SKYLINE,
        Some(
            ProblemArchetype::VariableCirclesMaxSum
            | ProblemArchetype::VariableCirclesInPerimRect,
        ) => SolverId::CPU_NLP,
        Some(ProblemArchetype::EqualCirclesInCircle)
        | Some(ProblemArchetype::CongruentCirclesCountBound) => SolverId::CPU_CONCENTRIC,
        Some(ProblemArchetype::EqualCirclesInSquare) if problem.objective == Objective::MaxEqualRadius => {
            SolverId::CPU_NLP
        }
        _ if problem.objective == Objective::MaxRadiusSum => SolverId::CPU_NLP,
        _ if problem.objective == Objective::MinBins => SolverId::CPU_FFD,
        _ => SolverId::CPU_FORCE,
    }
}

pub fn validate_solution(fixture: &Fixture, solution: &Solution) -> bool {
    if fixture.expect.feasible == Some(false) {
        return !solution.feasible;
    }
    if !solution.feasible {
        return false;
    }
    if let Some(bins) = fixture.expect.bin_count {
        if solution.metrics.bin_count != bins {
            return false;
        }
    }
    if fixture.expect.objective_record.is_some() {
        if solution.metrics.objective_value <= 0.0 && solution.metrics.radius_sum <= 0.0 {
            return false;
        }
    }
    true
}

pub fn bound_gap(fixture: &Fixture, solution: &Solution) -> Option<f64> {
    fixture.expect.objective_record.map(|record| {
        let achieved = if solution.metrics.radius_sum > 0.0 {
            solution.metrics.radius_sum
        } else {
            solution.metrics.objective_value
        };
        (record - achieved).max(0.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_paths_unique() {
        let paths = all_paths();
        let mut seen = std::collections::HashSet::new();
        for p in &paths {
            assert!(seen.insert(*p), "duplicate fixture path: {p}");
        }
        assert_eq!(paths.len(), registry::ENTRIES.len());
    }

    #[test]
    fn variable_circles_unit_square_n26() {
        let f = load_by_path("circle_sphere_packing/variable_circles_max_sum/unit_square_n26")
            .unwrap();
        assert_eq!(
            f.problem.archetype,
            Some(ProblemArchetype::VariableCirclesMaxSum)
        );
        assert_eq!(f.problem.items.circle_count(), Some(26));
    }

    #[test]
    fn one_dim_ffd_tight() {
        let f = load_by_path("manufacturer_pallet_loading/one_dim_bin_packing/ffd_tight").unwrap();
        let p = problem_from_fixture(&f);
        let sol = pm_solver::solve(&p, solver_for_problem(&p)).unwrap();
        assert!(validate_solution(&f, &sol));
        assert_eq!(sol.metrics.bin_count, f.expect.bin_count.unwrap());
    }

    #[test]
    fn variable_circles_feasible_smoke() {
        let f = load_by_path("circle_sphere_packing/variable_circles_max_sum/unit_square_n26")
            .unwrap();
        let p = problem_from_fixture(&f);
        let sol = pm_solver::solve(&p, solver_for_problem(&p)).unwrap();
        assert!(validate_solution(&f, &sol));
    }

    #[test]
    fn cubes_in_cubes_smoke() {
        let f = load_by_path("manufacturer_pallet_loading/cubes_in_cubes/n8_side3").unwrap();
        let p = problem_from_fixture(&f);
        let sol = pm_solver::solve(&p, solver_for_problem(&p)).unwrap();
        assert!(sol.feasible);
        assert!(sol.metrics.count > 0);
    }

    #[test]
    fn concentric_ring_r34_r5() {
        let n = pm_geometry::circle::concentric_ring_count(34.0, 5.0);
        assert!(n >= 30 && n <= 40);
    }
}
