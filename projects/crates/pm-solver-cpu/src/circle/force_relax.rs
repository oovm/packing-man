use pm_geometry::circle::{all_feasible, container_area, density, packed_circle_area};
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemFamily, Solution,
    SolutionMetrics, SolveMeta,
};

const MAX_ITERS: u32 = 400;
const REPULSE: f64 = 0.15;
const DAMP: f64 = 0.85;

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

    let mut placements: Vec<Placement2d> = (0..count)
        .map(|id| {
            let angle = (id as f64) * 2.399963;
            let r = radius * (1.0 + (id as f64) * 0.3);
            Placement2d {
                id,
                cx: r * angle.cos(),
                cy: r * angle.sin(),
                radius,
            }
        })
        .collect();

    for _iter in 0..MAX_ITERS {
        let mut moved = false;
        for i in 0..placements.len() {
            let mut fx = 0.0;
            let mut fy = 0.0;
            for j in 0..placements.len() {
                if i == j {
                    continue;
                }
                let dx = placements[i].cx - placements[j].cx;
                let dy = placements[i].cy - placements[j].cy;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-9);
                let overlap = placements[i].radius + placements[j].radius - dist;
                if overlap > 0.0 {
                    fx += (dx / dist) * overlap;
                    fy += (dy / dist) * overlap;
                    moved = true;
                }
            }
            push_inside_container(
                &mut placements[i],
                &problem.container,
                &mut fx,
                &mut fy,
            );
            placements[i].cx += fx * REPULSE;
            placements[i].cy += fy * REPULSE;
        }
        if !moved {
            break;
        }
        for p in &mut placements {
            p.cx *= DAMP;
            p.cy *= DAMP;
        }
    }

    let feasible = all_feasible(&placements, &problem.container);
    Some(build_solution(problem, placements, feasible, "cpu_force_relax", MAX_ITERS))
}

fn push_inside_container(
    p: &mut Placement2d,
    container: &ContainerModel,
    fx: &mut f64,
    fy: &mut f64,
) {
    match container {
        ContainerModel::Circle { radius } => {
            let dist = (p.cx * p.cx + p.cy * p.cy).sqrt().max(1e-9);
            let overflow = dist + p.radius - *radius;
            if overflow > 0.0 {
                *fx -= (p.cx / dist) * overflow;
                *fy -= (p.cy / dist) * overflow;
            }
        }
        ContainerModel::Rectangle { width, height } => {
            if p.cx - p.radius < 0.0 {
                *fx += p.radius - p.cx;
            }
            if p.cx + p.radius > *width {
                *fx -= p.cx + p.radius - *width;
            }
            if p.cy - p.radius < 0.0 {
                *fy += p.radius - p.cy;
            }
            if p.cy + p.radius > *height {
                *fy -= p.cy + p.radius - *height;
            }
        }
        _ => {}
    }
}

fn empty_solution(problem: &Problem) -> Solution {
    build_solution(problem, vec![], true, "cpu_force_relax", 0)
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
        && matches!(problem.items.model, ItemModel::Circle { .. })
}
