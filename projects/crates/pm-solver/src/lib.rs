//! 求解注册表与调度。

mod compat;

use std::time::Instant;

use pm_types::{
    normalize_problem, Backend, PmError, PmResult, Problem, ProblemFamily, Solution, SolverId,
};

pub fn solve(problem: &Problem, solver: SolverId) -> PmResult<Solution> {
    let mut normalized = problem.clone();
    normalize_problem(&mut normalized);

    compat::validate(&normalized, solver)?;

    match normalized.family {
        ProblemFamily::DistributorPalletLoading | ProblemFamily::ConvexRegionPacking => {
            return Err(PmError::UnsupportedFamily(normalized.family));
        }
        ProblemFamily::CircleSpherePacking | ProblemFamily::ManufacturerPalletLoading => {}
    }

    let start = Instant::now();
    let solution = match solver.backend {
        Backend::Cpu => pm_solver_cpu::solve(&normalized, solver),
        Backend::Gpu => pm_solver_gpu::solve(&normalized, solver),
    };
    let elapsed = start.elapsed().as_millis() as u64;

    match solution {
        Some(mut s) => {
            s.meta.elapsed_ms = elapsed;
            Ok(s)
        }
        None => Err(PmError::UnsupportedSolver {
            family: normalized.family,
            algorithm: format!("{:?}", solver.algorithm),
        }),
    }
}

pub struct Registry;

impl Registry {
    pub fn solve(problem: &Problem, solver: SolverId) -> PmResult<Solution> {
        solve(problem, solver)
    }
}
