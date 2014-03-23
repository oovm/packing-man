//! CPU 求解算法。

pub mod bin;
pub mod box3d;
pub mod circle;
pub mod common;
pub mod strip;

use pm_types::{normalize_problem, Problem, ProblemFamily, Solution, SolverId};

pub fn solve(problem: &Problem, solver: SolverId) -> Option<Solution> {
    let mut p = problem.clone();
    normalize_problem(&mut p);
    match p.family {
        ProblemFamily::CircleSpherePacking => circle::solve(&p, solver.algorithm),
        ProblemFamily::ManufacturerPalletLoading => {
            bin::solve(&p, solver.algorithm)
                .or_else(|| strip::solve(&p, solver.algorithm))
                .or_else(|| box3d::solve(&p, solver.algorithm))
        }
        _ => None,
    }
}
