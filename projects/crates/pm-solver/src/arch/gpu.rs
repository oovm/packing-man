use pm_checkpoint::ResumeSession;
use pm_types::{AlgorithmKind, Problem, Solution, SolverId};

use crate::circle;

pub struct GpuArch;

impl super::SolverArch for GpuArch {
    fn solve_with_resume(
        &self,
        problem: &Problem,
        solver: SolverId,
        resume: Option<ResumeSession>,
    ) -> Option<Solution> {
        if solver.algorithm != AlgorithmKind::GpuForceRelaxation {
            return None;
        }
        let circle_session = resume.map(|s| circle::force_relax::RelaxSession {
            initial_placements: Some(s.initial_placements),
            iter_budget: s.iter_budget,
            total_iterations_before: s.total_iterations_before,
        });
        let mut solution = circle::force_relax::solve_with_session(problem, circle_session)?;
        solution.meta.backend = "gpu".to_string();
        solution.meta.algorithm = "gpu_force_relax".to_string();
        Some(solution)
    }
}

pub fn gpu_available() -> bool {
    cfg!(feature = "gpu")
}
