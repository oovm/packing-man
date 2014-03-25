pub mod skyline;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::SkylineStrip => skyline::solve(problem),
        _ => None,
    }
}
