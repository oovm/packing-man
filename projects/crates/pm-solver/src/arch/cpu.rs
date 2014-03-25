use pm_checkpoint::ResumeSession;
use pm_types::{normalize_problem, Problem, ProblemFamily, Solution, SolverId};

use crate::circle;

pub struct CpuArch;

impl super::SolverArch for CpuArch {
    fn solve_with_resume(
        &self,
        problem: &Problem,
        solver: SolverId,
        resume: Option<ResumeSession>,
    ) -> Option<Solution> {
        let mut p = problem.clone();
        normalize_problem(&mut p);
        let circle_session = resume.map(|s| circle::force_relax::RelaxSession {
            initial_placements: Some(s.initial_placements),
            iter_budget: s.iter_budget,
            total_iterations_before: s.total_iterations_before,
        });
        match p.family {
            ProblemFamily::CircleSpherePacking => {
                circle::solve_with_resume(&p, solver.algorithm, circle_session)
            }
            ProblemFamily::ManufacturerPalletLoading => {
                crate::one_dim::solve(&p, solver.algorithm)
                    .or_else(|| crate::strip::solve(&p, solver.algorithm))
                    .or_else(|| crate::box3d::solve(&p, solver.algorithm))
            }
            _ => None,
        }
    }
}
