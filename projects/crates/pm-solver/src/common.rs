use pm_geometry::circle::{
    container_area, density, min_center_distance, min_pairwise_distance, packed_circle_area,
    radius_sum,
};
use pm_geometry::orthogonal::strip_height;
use pm_types::{
    Objective, Placement2d, PlacementRect, Problem, Solution, SolutionMetrics, SolveMeta,
};

pub fn build_circle_solution(
    problem: &Problem,
    placements: Vec<Placement2d>,
    feasible: bool,
    algorithm: &str,
    iterations: u32,
) -> Solution {
    let area = container_area(&problem.container);
    let count = placements.len() as u32;
    let dens = density(&placements, &problem.container);
    let packed = packed_circle_area(&placements);
    let r_sum = radius_sum(&placements);
    let min_dist = match problem.objective {
        Objective::MaxMinDistance => min_center_distance(&placements),
        _ => min_pairwise_distance(&placements),
    };
    let equal_r = placements.first().map(|p| p.radius).unwrap_or(0.0);
    let objective_value = match problem.objective {
        Objective::MaxRadiusSum | Objective::MaxTotalPerimeter => r_sum,
        Objective::MaxMinDistance => min_dist,
        Objective::MaxEqualRadius => equal_r,
        _ => count as f64,
    };
    Solution {
        family: problem.family,
        placements,
        rect_placements: vec![],
        placements_3d: vec![],
        bins_1d: vec![],
        bins: vec![],
        metrics: SolutionMetrics {
            count,
            density: dens,
            container_area: area,
            packed_area: packed,
            radius_sum: r_sum,
            min_pairwise_distance: min_dist,
            equal_radius: equal_r,
            objective_value,
            ..SolutionMetrics::default()
        },
        feasible,
        meta: SolveMeta {
            algorithm: algorithm.to_string(),
            backend: "cpu".to_string(),
            iterations,
            elapsed_ms: 0,
        },
    }
}

pub fn build_strip_solution(
    problem: &Problem,
    placements: Vec<PlacementRect>,
    feasible: bool,
    algorithm: &str,
) -> Solution {
    let h = strip_height(&placements);
    let n = placements.len() as u32;
    let container_area = pm_geometry::circle::container_area(&problem.container);
    Solution {
        family: problem.family,
        placements: vec![],
        rect_placements: placements,
        placements_3d: vec![],
        bins_1d: vec![],
        bins: vec![],
        metrics: SolutionMetrics {
            count: n,
            strip_height: h,
            objective_value: h,
            container_area,
            ..SolutionMetrics::default()
        },
        feasible,
        meta: SolveMeta {
            algorithm: algorithm.to_string(),
            backend: "cpu".to_string(),
            iterations: 0,
            elapsed_ms: 0,
        },
    }
}
