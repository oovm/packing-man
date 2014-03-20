pub mod force_relax;
pub mod greedy;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::GreedyInsertion => greedy::solve(problem),
        AlgorithmKind::ForceRelaxation => force_relax::solve(problem),
        _ => None,
    }
}
