pub mod analytical_ring;
pub mod force_relax;
pub mod greedy;
pub mod variable_relax;

use pm_types::{AlgorithmKind, Problem, Solution};

pub fn solve(problem: &Problem, algorithm: AlgorithmKind) -> Option<Solution> {
    solve_with_resume(problem, algorithm, None)
}

pub fn solve_with_resume(
    problem: &Problem,
    algorithm: AlgorithmKind,
    session: Option<force_relax::RelaxSession>,
) -> Option<Solution> {
    match algorithm {
        AlgorithmKind::GreedyInsertion => greedy::solve(problem),
        AlgorithmKind::ForceRelaxation => force_relax::solve_with_session(problem, session),
        AlgorithmKind::AnalyticalConcentricRing => analytical_ring::solve(problem),
        AlgorithmKind::NlpLocalSearch => {
            let mapped = session.map(|s| variable_relax::RelaxSession {
                initial_placements: s.initial_placements,
                iter_budget: s.iter_budget,
                total_iterations_before: s.total_iterations_before,
            });
            variable_relax::solve_with_session(problem, mapped)
        }
        _ => None,
    }
}
