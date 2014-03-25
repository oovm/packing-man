use pm_geometry::circle::all_feasible;
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemFamily,
};

use crate::common::build_circle_solution;

pub const DEFAULT_MAX_ITERS: u32 = 400;
const REPULSE: f64 = 0.15;
const DAMP: f64 = 0.85;

#[derive(Debug, Clone, Default)]
pub struct RelaxSession {
    pub initial_placements: Option<Vec<Placement2d>>,
    pub iter_budget: u32,
    pub total_iterations_before: u32,
}

impl RelaxSession {
    pub fn fresh() -> Self {
        Self {
            initial_placements: None,
            iter_budget: DEFAULT_MAX_ITERS,
            total_iterations_before: 0,
        }
    }
}

pub fn solve(problem: &Problem) -> Option<pm_types::Solution> {
    solve_with_session(problem, None)
}

pub fn solve_with_session(
    problem: &Problem,
    session: Option<RelaxSession>,
) -> Option<pm_types::Solution> {
    if problem.objective == Objective::MaxRadiusSum {
        let mapped = session.map(|s| super::variable_relax::RelaxSession {
            initial_placements: s.initial_placements,
            iter_budget: s.iter_budget,
            total_iterations_before: s.total_iterations_before,
        });
        return super::variable_relax::solve_with_session(problem, mapped);
    }
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

    let session = session.unwrap_or_else(RelaxSession::fresh);
    let max_iters = session.iter_budget.max(1);
    let mut placements = session
        .initial_placements
        .filter(|p| p.len() == count as usize)
        .unwrap_or_else(|| spiral_init(radius, count));

    let mut ran_iters = 0u32;
    for _iter in 0..max_iters {
        ran_iters += 1;
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

    let total_iters = session.total_iterations_before + ran_iters;
    let feasible = all_feasible(&placements, &problem.container);
    Some(build_circle_solution(
        problem,
        placements,
        feasible,
        "cpu_force_relax",
        total_iters,
    ))
}

fn spiral_init(radius: f64, count: u32) -> Vec<Placement2d> {
    (0..count)
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
        .collect()
}

pub(crate) fn push_inside_container(
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

fn empty_solution(problem: &Problem) -> pm_types::Solution {
    build_circle_solution(problem, vec![], true, "cpu_force_relax", 0)
}

pub fn supports(problem: &Problem) -> bool {
    problem.family == ProblemFamily::CircleSpherePacking
        && matches!(problem.objective, Objective::MaxCount)
        && matches!(problem.items.model, ItemModel::Circle { .. })
}
