use std::fs;
use std::path::Path;

use pm_types::{normalize_problem, PmError, PmResult, Problem, Solution, SolverId};

pub const CHECKPOINT_VERSION: u32 = 1;

/// 可序列化断点文件（JSON）。
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Checkpoint {
    pub version: u32,
    pub problem: Problem,
    pub solver: SolverId,
    /// 已累计迭代步数（跨多次续跑）。
    pub total_iterations: u32,
    /// 单次会话默认迭代预算（续跑时未指定则用此值）。
    pub iter_budget: u32,
    pub solution: Solution,
}

impl Checkpoint {
    pub fn new(
        problem: Problem,
        solver: SolverId,
        solution: Solution,
        total_iterations: u32,
        iter_budget: u32,
    ) -> Self {
        Self {
            version: CHECKPOINT_VERSION,
            problem,
            solver,
            total_iterations,
            iter_budget,
            solution,
        }
    }

    pub fn from_run(
        problem: &Problem,
        solver: SolverId,
        solution: &Solution,
        iter_budget: u32,
    ) -> Self {
        let mut p = problem.clone();
        normalize_problem(&mut p);
        Self::new(
            p,
            solver,
            solution.clone(),
            solution.meta.iterations,
            iter_budget,
        )
    }

    pub fn validate(&self, problem: &Problem, solver: SolverId) -> PmResult<()> {
        if self.version != CHECKPOINT_VERSION {
            return Err(PmError::InvalidProblem(format!(
                "checkpoint version {} unsupported (want {})",
                self.version,
                CHECKPOINT_VERSION
            )));
        }

        let mut expected = problem.clone();
        normalize_problem(&mut expected);
        let mut stored = self.problem.clone();
        normalize_problem(&mut stored);
        if expected != stored {
            return Err(PmError::IncompatibleProblem(
                "checkpoint problem does not match request".into(),
            ));
        }

        if self.solver != solver {
            return Err(PmError::IncompatibleProblem(format!(
                "checkpoint solver {:?} does not match {:?}",
                self.solver.algorithm,
                solver.algorithm
            )));
        }

        if self.solution.placements.is_empty() && self.solution.rect_placements.is_empty() {
            return Err(PmError::InvalidProblem(
                "checkpoint has no placement state to resume".into(),
            ));
        }

        Ok(())
    }

    pub fn to_json(&self) -> PmResult<String> {
        serde_json::to_string_pretty(self).map_err(|e| PmError::Serde(e.to_string()))
    }

    pub fn from_json(json: &str) -> PmResult<Self> {
        serde_json::from_str(json).map_err(|e| PmError::Serde(e.to_string()))
    }

    pub fn write_path(&self, path: impl AsRef<Path>) -> PmResult<()> {
        fs::write(path, self.to_json()?).map_err(|e| PmError::Serde(e.to_string()))
    }

    pub fn read_path(path: impl AsRef<Path>) -> PmResult<Self> {
        let text = fs::read_to_string(path).map_err(|e| PmError::Serde(e.to_string()))?;
        Self::from_json(&text)
    }
}

#[cfg(test)]
mod tests {
    use pm_types::{Problem, SolverId};

    use super::*;

    #[test]
    fn roundtrip_json() {
        let problem = Problem::circles_in_disk(1.0, 10.0, 5);
        let solution = pm_solver::solve(&problem, SolverId::CPU_FORCE).unwrap();
        let cp = Checkpoint::from_run(&problem, SolverId::CPU_FORCE, &solution, 400);
        let parsed = Checkpoint::from_json(&cp.to_json().unwrap()).unwrap();
        assert_eq!(parsed.total_iterations, solution.meta.iterations);
        assert_eq!(parsed.solution.placements.len(), 5);
    }
}
