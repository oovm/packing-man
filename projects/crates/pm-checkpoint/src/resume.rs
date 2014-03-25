use pm_types::{AlgorithmKind, Backend, Placement2d, Problem, Solution, SolverId};

use crate::Checkpoint;

/// 单次续跑会话：从断点布局出发再跑若干迭代。
#[derive(Debug, Clone)]
pub struct ResumeSession {
    pub initial_placements: Vec<Placement2d>,
    pub iter_budget: u32,
    pub total_iterations_before: u32,
}

/// 求解请求：可选断点 + 本次迭代预算。
#[derive(Debug, Clone, Default)]
pub struct SolveResume {
    pub checkpoint: Option<Checkpoint>,
    pub iter_budget: Option<u32>,
}

impl SolveResume {
    pub fn from_checkpoint(checkpoint: Checkpoint, iter_budget: Option<u32>) -> Self {
        Self {
            checkpoint: Some(checkpoint),
            iter_budget,
        }
    }
}

pub fn is_resumable(solver: SolverId) -> bool {
    match (solver.backend, solver.algorithm) {
        (Backend::Cpu, AlgorithmKind::ForceRelaxation)
        | (Backend::Cpu, AlgorithmKind::NlpLocalSearch)
        | (Backend::Gpu, AlgorithmKind::GpuForceRelaxation) => true,
        _ => false,
    }
}

pub fn default_iter_budget(solver: SolverId, problem: &Problem) -> u32 {
    if problem.objective == pm_types::Objective::MaxRadiusSum {
        return 600;
    }
    match solver.algorithm {
        AlgorithmKind::NlpLocalSearch => 600,
        AlgorithmKind::ForceRelaxation | AlgorithmKind::GpuForceRelaxation => 400,
        _ => 400,
    }
}

impl ResumeSession {
    pub fn from_checkpoint(checkpoint: &Checkpoint, iter_budget: Option<u32>) -> Self {
        Self {
            initial_placements: checkpoint.solution.placements.clone(),
            iter_budget: iter_budget.unwrap_or(checkpoint.iter_budget),
            total_iterations_before: checkpoint.total_iterations,
        }
    }
}

pub fn build_checkpoint_after_run(
    problem: &Problem,
    solver: SolverId,
    solution: &Solution,
    iter_budget: u32,
) -> Checkpoint {
    Checkpoint::from_run(problem, solver, solution, iter_budget)
}
