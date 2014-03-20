use pm_geometry::circle::{all_feasible, container_area, density, packed_circle_area};
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemFamily, Solution,
    SolutionMetrics, SolveMeta,
};

pub fn solve(problem: &Problem) -> Option<Solution> {
    if problem.family != ProblemFamily::CircleSpherePacking {
        return None;
    }
    let (radius, count) = match &problem.items.model {
        ItemModel::Circle { radius, count } => (*radius, *count),
        _ => return None,
    };
    if count == 0 {
        return Some(empty_solution(problem));
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
    Some(build_solution(problem, placements, feasible, "cpu_greedy", 0))
}

fn empty_solution(problem: &Problem) -> Solution {
    build_solution(problem, vec![], true, "cpu_greedy", 0)
}

fn build_solution(
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
    Solution {
        family: problem.family,
        placements,
        metrics: SolutionMetrics {
            count,
            density: dens,
            container_area: area,
            packed_area: packed,
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

pub fn supports(problem: &Problem) -> bool {
    problem.family == ProblemFamily::CircleSpherePacking
        && matches!(problem.objective, Objective::MaxCount)
        && matches!(
            problem.items.model,
            ItemModel::Circle { .. }
        )
        && matches!(
            problem.container,
            ContainerModel::Circle { .. } | ContainerModel::Rectangle { .. }
        )
}
