//! 按 `Backend` 分派的求解架构（CPU / GPU）。

mod cpu;
mod gpu;

use pm_checkpoint::ResumeSession;
use pm_types::{Backend, Problem, Solution, SolverId};

pub use cpu::CpuArch;
pub use gpu::{gpu_available, GpuArch};

/// 求解后端：CPU 全算法族，GPU 仅力松弛（V1 由 CPU 模拟并行步）。
pub trait SolverArch: Send + Sync {
    fn solve_with_resume(
        &self,
        problem: &Problem,
        solver: SolverId,
        resume: Option<ResumeSession>,
    ) -> Option<Solution>;
}

pub fn arch_for(backend: Backend) -> &'static dyn SolverArch {
    match backend {
        Backend::Cpu => &CpuArch,
        Backend::Gpu => &GpuArch,
    }
}

pub fn solve_with_resume(
    problem: &Problem,
    solver: SolverId,
    resume: Option<ResumeSession>,
) -> Option<Solution> {
    arch_for(solver.backend).solve_with_resume(problem, solver, resume)
}
