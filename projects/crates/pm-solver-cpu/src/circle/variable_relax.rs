use pm_geometry::circle::{all_feasible, radius_sum};
use pm_types::{
    ContainerModel, ItemModel, Objective, Placement2d, Problem, ProblemFamily,
};

use super::force_relax::push_inside_container;
use crate::common::build_circle_solution;

const MAX_ITERS: u32 = 600;
const REPULSE: f64 = 0.12;
const GROW: f64 = 0.02;

pub fn solve(problem: &Problem) -> Option<pm_types::Solution> {
    if problem.family != ProblemFamily::CircleSpherePacking {
        return None;
    }
    let count = match &problem.items.model {
        ItemModel::VariableCircles { count } => *count,
        ItemModel::Circle { count, .. } if problem.objective == Objective::MaxRadiusSum => *count,
        _ => return None,
    };
    if count == 0 {
        return Some(build_circle_solution(problem, vec![], true, "variable_relax", 0));
    }

    let init_r = 0.05;
    let mut placements: Vec<Placement2d> = (0..count)
        .map(|id| {
            let angle = (id as f64) * 2.399963;
            let rr = 0.3 + (id as f64) * 0.02;
            Placement2d {
                id,
                cx: 0.5 + rr * angle.cos(),
                cy: 0.5 + rr * angle.sin(),
                radius: init_r,
            }
        })
        .collect();

    for iter in 0..MAX_ITERS {
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
            push_inside_container(&mut placements[i], &problem.container, &mut fx, &mut fy);
            placements[i].cx += fx * REPULSE;
            placements[i].cy += fy * REPULSE;
        }
        if problem.objective == Objective::MaxRadiusSum && iter % 5 == 0 {
            for i in 0..placements.len() {
                let trial = placements[i].radius + GROW;
                let mut test = placements[i];
                test.radius = trial;
                let mut fx = 0.0;
                let mut fy = 0.0;
                push_inside_container(&mut test, &problem.container, &mut fx, &mut fy);
                if fx.abs() < 1e-6 && fy.abs() < 1e-6 {
                    let ok = placements.iter().all(|other| {
                        if other.id == test.id {
                            return true;
                        }
                        let dx = test.cx - other.cx;
                        let dy = test.cy - other.cy;
                        let dist = (dx * dx + dy * dy).sqrt();
                        dist >= test.radius + other.radius - 1e-9
                    });
                    if ok {
                        placements[i].radius = trial;
                    }
                }
            }
        }
        if !moved && iter > 50 {
            break;
        }
    }

    normalize_to_unit_square(&mut placements, &problem.container);
    let feasible = all_feasible(&placements, &problem.container);
    let mut sol = build_circle_solution(problem, placements, feasible, "variable_relax", MAX_ITERS);
    sol.metrics.radius_sum = radius_sum(&sol.placements);
    sol.metrics.objective_value = sol.metrics.radius_sum;
    Some(sol)
}

fn normalize_to_unit_square(placements: &mut [Placement2d], container: &ContainerModel) {
    if !matches!(container, ContainerModel::Rectangle { width: 1.0, height: 1.0 }) {
        return;
    }
    for p in placements.iter_mut() {
        if p.cx > 1.0 || p.cy > 1.0 {
            p.cx = p.cx.min(1.0 - p.radius).max(p.radius);
            p.cy = p.cy.min(1.0 - p.radius).max(p.radius);
        }
    }
}

pub fn supports(problem: &Problem) -> bool {
    problem.family == ProblemFamily::CircleSpherePacking
        && problem.objective == Objective::MaxRadiusSum
        && matches!(
            problem.items.model,
            ItemModel::VariableCircles { .. } | ItemModel::Circle { .. }
        )
}
