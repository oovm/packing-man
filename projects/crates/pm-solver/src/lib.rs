//! 求解注册表与调度。

use std::time::Instant;

use pm_types::{
    Backend, PmError, PmResult, Problem, ProblemFamily, Solution, SolverId,
};

pub struct Registry;

impl Registry {
    pub fn solve(problem: &Problem, solver: SolverId) -> PmResult<Solution> {
        if problem.family != solver.family {
            return Err(PmError::UnsupportedSolver {
                family: problem.family,
                algorithm: format!("{:?}", solver.algorithm),
            });
        }

        match problem.family {
            ProblemFamily::ManufacturerPalletLoading
            | ProblemFamily::DistributorPalletLoading
            | ProblemFamily::ConvexRegionPacking => {
                return Err(PmError::UnsupportedFamily(problem.family));
            }
            ProblemFamily::CircleSpherePacking => {}
        }

        let start = Instant::now();
        let solution = match solver.backend {
            Backend::Cpu => pm_solver_cpu::solve(problem, solver),
            Backend::Gpu => pm_solver_gpu::solve(problem, solver),
        };
        let elapsed = start.elapsed().as_millis() as u64;

        match solution {
            Some(mut s) => {
                s.meta.elapsed_ms = elapsed;
                Ok(s)
            }
            None => Err(PmError::UnsupportedSolver {
                family: problem.family,
                algorithm: format!("{:?}", solver.algorithm),
            }),
        }
    }
}

pub fn solve(problem: &Problem, solver: SolverId) -> PmResult<Solution> {
    Registry::solve(problem, solver)
}
