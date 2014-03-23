use pm_geometry::circle::all_feasible;
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemFamily,
};

use crate::common::build_circle_solution;

pub fn solve(problem: &Problem) -> Option<pm_types::Solution> {
    if problem.family != ProblemFamily::CircleSpherePacking {
        return None;
    }
    let (radius, count) = match &problem.items.model {
        ItemModel::Circle { radius, count } => (*radius, *count),
        ItemModel::IdenticalCircles {
            radius: Some(r),
            count,
        } => (*r, *count),
        _ => return None,
    };
    if count == 0 {
        return Some(build_circle_solution(problem, vec![], true, "cpu_greedy", 0));
    }

    let mut placements = Vec::new();
    let ring_step = radius * 2.05;
    let mut ring = 0u32;
    while placements.len() < count as usize {
        if ring == 0 {
            placements.push(Placement2d {
                id: placements.len() as u32,
                cx: 0.0,
                cy: 0.0,
                radius,
            });
            ring += 1;
            continue;
        }
        let n = (6 * ring).max(1);
        let r = ring as f64 * ring_step;
        for k in 0..n {
            if placements.len() >= count as usize {
                break;
            }
            let angle = (k as f64 / n as f64) * std::f64::consts::TAU;
            placements.push(Placement2d {
                id: placements.len() as u32,
                cx: r * angle.cos(),
                cy: r * angle.sin(),
                radius,
            });
        }
        ring += 1;
        if ring > 200 {
            break;
        }
    }

    let feasible = all_feasible(&placements, &problem.container);
    Some(build_circle_solution(problem, placements, feasible, "cpu_greedy", 0))
}

pub fn supports(problem: &Problem) -> bool {
    problem.family == ProblemFamily::CircleSpherePacking
        && matches!(problem.objective, Objective::MaxCount)
        && matches!(
            problem.items.model,
            ItemModel::Circle { .. } | ItemModel::IdenticalCircles { .. }
        )
        && matches!(
            problem.container,
            ContainerModel::Circle { .. } | ContainerModel::Rectangle { .. }
        )
}
