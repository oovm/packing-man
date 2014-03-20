//! GPU 求解（V1：并行力步由 CPU 模拟，feature gpu 预留 wgpu 接入）。

use pm_types::{AlgorithmKind, Problem, Solution, SolverId};

pub fn solve(problem: &Problem, solver: SolverId) -> Option<Solution> {
    if solver.algorithm != AlgorithmKind::GpuForceRelaxation {
        return None;
    }
    let mut solution = pm_solver_cpu::circle::force_relax::solve(problem)?;
    solution.meta.backend = "gpu".to_string();
    solution.meta.algorithm = "gpu_force_relax".to_string();
    Some(solution)
}

pub fn gpu_available() -> bool {
    cfg!(feature = "gpu")
}
