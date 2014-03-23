pub mod analytical_ring;
pub mod force_relax;
pub mod greedy;
pub mod variable_relax;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::GreedyInsertion => greedy::solve(problem),
        AlgorithmKind::ForceRelaxation => force_relax::solve(problem),
        AlgorithmKind::AnalyticalConcentricRing => analytical_ring::solve(problem),
        AlgorithmKind::NlpLocalSearch => variable_relax::solve(problem),
        _ => None,
    }
}
