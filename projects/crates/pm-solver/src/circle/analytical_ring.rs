use pm_geometry::circle::{all_feasible, concentric_ring_count};
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemArchetype, ProblemFamily,
};

use crate::common::build_circle_solution;

pub fn solve(problem: &Problem) -> Option<pm_types::Solution> {
    if problem.family != ProblemFamily::CircleSpherePacking {
        return None;
    }
    let (item_r, _) = circle_params(problem)?;
    let container_r = match &problem.container {
        ContainerModel::Circle { radius } => *radius,
        _ => return None,
    };
    let count = concentric_ring_count(container_r, item_r);
    if count == 0 {
        return Some(build_circle_solution(problem, vec![], false, "concentric_ring", 0));
    }
    let placements = greedy_ring_layout(container_r, item_r, count);
    let feasible = all_feasible(&placements, &problem.container);
    let mut sol = build_circle_solution(problem, placements, feasible, "concentric_ring", 1);
    sol.metrics.count = count;
    sol.metrics.objective_value = count as f64;
    if problem.archetype == Some(ProblemArchetype::CongruentCirclesCountBound) {
        sol.metrics.bound_gap = 0.0;
    }
    Some(sol)
}

fn circle_params(problem: &Problem) -> Option<(f64, u32)> {
    match &problem.items.model {
        ItemModel::Circle { radius, count } => Some((*radius, *count)),
        ItemModel::IdenticalCircles { radius: Some(r), count } => Some((*r, *count)),
        _ => None,
    }
}

fn greedy_ring_layout(container_r: f64, item_r: f64, count: u32) -> Vec<Placement2d> {
    let mut placements = Vec::new();
    let step = item_r * 2.05;
    let mut ring = 0u32;
    while placements.len() < count as usize {
        if ring == 0 {
            placements.push(Placement2d {
                id: 0,
                cx: 0.0,
                cy: 0.0,
                radius: item_r,
            });
            ring += 1;
            continue;
        }
        let n = (6 * ring).max(1);
        let r = ring as f64 * step;
        if r + item_r > container_r {
            break;
        }
        for k in 0..n {
            if placements.len() >= count as usize {
                break;
            }
            let angle = (k as f64 / n as f64) * std::f64::consts::TAU;
            placements.push(Placement2d {
                id: placements.len() as u32,
                cx: r * angle.cos(),
                cy: r * angle.sin(),
                radius: item_r,
            });
        }
        ring += 1;
    }
    placements
}

pub fn supports(problem: &Problem) -> bool {
    problem.family == ProblemFamily::CircleSpherePacking
        && matches!(
            problem.objective,
            Objective::MaxCount | Objective::Feasibility
        )
        && matches!(problem.container, ContainerModel::Circle { .. })
}
