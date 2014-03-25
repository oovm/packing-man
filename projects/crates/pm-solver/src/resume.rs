use std::time::Instant;

use pm_checkpoint::{
    build_checkpoint_after_run, default_iter_budget, is_resumable, Checkpoint, ResumeSession,
    SolveResume,
};
use pm_types::{normalize_problem, PmError, PmResult, Problem, Solution, SolverId};

use crate::arch;
use crate::compat;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SolveResumeResult {
    pub solution: Solution,
    pub checkpoint: Checkpoint,
}

pub fn solve_resume(
    problem: &Problem,
    solver: SolverId,
    request: SolveResume,
) -> PmResult<SolveResumeResult> {
    let mut normalized = problem.clone();
    normalize_problem(&mut normalized);
    compat::validate(&normalized, solver)?;

    let iter_budget = request
        .iter_budget
        .or_else(|| request.checkpoint.as_ref().map(|c| c.iter_budget))
        .unwrap_or_else(|| default_iter_budget(solver, &normalized));

    let resume_session = if let Some(checkpoint) = &request.checkpoint {
        checkpoint.validate(&normalized, solver)?;
        if !is_resumable(solver) {
            return Err(PmError::UnsupportedSolver {
                family: normalized.family,
                algorithm: format!("{:?} does not support checkpoint resume", solver.algorithm),
            });
        }
        Some(ResumeSession::from_checkpoint(checkpoint, Some(iter_budget)))
    } else {
        None
    };

    let start = Instant::now();
    let solution = arch::solve_with_resume(&normalized, solver, resume_session);
    let elapsed = start.elapsed().as_millis() as u64;

    match solution {
        Some(mut s) => {
            s.meta.elapsed_ms = elapsed;
            let checkpoint = build_checkpoint_after_run(&normalized, solver, &s, iter_budget);
            Ok(SolveResumeResult {
                solution: s,
                checkpoint,
            })
        }
        None => Err(PmError::UnsupportedSolver {
            family: normalized.family,
            algorithm: format!("{:?}", solver.algorithm),
        }),
    }
}

#[cfg(test)]
mod tests {
    use pm_checkpoint::SolveResume;
    use pm_types::{Problem, SolverId};

    use super::solve_resume;

    #[test]
    fn resume_increases_iteration_count() {
        let problem = Problem::circles_in_disk(1.0, 10.0, 19);
        let first = solve_resume(
            &problem,
            SolverId::CPU_FORCE,
            SolveResume {
                checkpoint: None,
                iter_budget: Some(50),
            },
        )
        .unwrap();
        assert!(first.solution.meta.iterations >= 1);

        let second = solve_resume(
            &problem,
            SolverId::CPU_FORCE,
            SolveResume {
                checkpoint: Some(first.checkpoint),
                iter_budget: Some(50),
            },
        )
        .unwrap();
        assert!(second.solution.meta.iterations > first.solution.meta.iterations);
        assert_eq!(second.solution.placements.len(), 19);
    }
}
