//! 求解注册表、架构分派与算法实现。

mod arch;
mod box3d;
mod circle;
mod common;
mod compat;
mod one_dim;
mod resume;
mod strip;

use std::time::Instant;

use pm_checkpoint::SolveResume;
use pm_types::{
    normalize_problem, PmError, PmResult, Problem, ProblemFamily, Solution, SolverId,
};

pub use arch::{gpu_available, SolverArch};
pub use resume::{solve_resume, SolveResumeResult};

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
    let solution = arch::solve_with_resume(&normalized, solver, None);
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

pub fn solve_with_checkpoint(
    problem: &Problem,
    solver: SolverId,
    request: SolveResume,
) -> PmResult<SolveResumeResult> {
    solve_resume(problem, solver, request)
}

pub struct Registry;

impl Registry {
    pub fn solve(problem: &Problem, solver: SolverId) -> PmResult<Solution> {
        solve(problem, solver)
    }

    pub fn solve_with_checkpoint(
        problem: &Problem,
        solver: SolverId,
        request: SolveResume,
    ) -> PmResult<SolveResumeResult> {
        solve_with_checkpoint(problem, solver, request)
    }
}
