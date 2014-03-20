//! CPU 求解算法。

pub mod circle;

use pm_types::{Problem, Solution, SolverId};

pub fn solve(problem: &Problem, solver: SolverId) -> Option<Solution> {
    match solver.family {
        pm_types::ProblemFamily::CircleSpherePacking => {
            circle::solve(problem, solver.algorithm)
        }
        _ => None,
    }
}
