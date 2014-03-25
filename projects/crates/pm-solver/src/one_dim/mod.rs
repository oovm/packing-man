pub mod ffd_bfd;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::FirstFitDecreasing => ffd_bfd::solve_ffd(problem),
        AlgorithmKind::BestFitDecreasing => ffd_bfd::solve_bfd(problem),
        _ => None,
    }
}
