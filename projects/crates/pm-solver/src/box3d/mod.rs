pub mod extreme_point;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::ExtremePointBlf => extreme_point::solve(problem),
        _ => None,
    }
}
