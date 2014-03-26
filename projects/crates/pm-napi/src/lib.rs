//! Node N-API：JSON 求解 + SVG + 断点续跑。

use napi::bindgen_prelude::*;
use napi_derive::napi;
use pm_checkpoint::{Checkpoint, SolveResume};
use pm_solver::{solve, solve_resume};
use pm_svg::render;
use pm_types::{Problem, SolverId};

#[napi]
pub fn pm_version_code() -> u32 {
    0_004_000
}

#[napi]
pub fn solve_packing_json(problem_json: String, solver_json: String) -> Result<String> {
    let mut problem: Problem = serde_json::from_str(&problem_json)
        .map_err(|e| Error::from_reason(format!("problem json: {e}")))?;
    pm_types::normalize_problem(&mut problem);
    let solver: SolverId = serde_json::from_str(&solver_json)
        .map_err(|e| Error::from_reason(format!("solver json: {e}")))?;
    let solution = solve(&problem, solver).map_err(|e| Error::from_reason(format!("{e}")))?;
    serde_json::to_string(&solution).map_err(|e| Error::from_reason(format!("{e}")))
}

#[napi]
pub fn solve_packing_resume_json(
    problem_json: String,
    solver_json: String,
    checkpoint_json: Option<String>,
    iter_budget: Option<u32>,
) -> Result<String> {
    let mut problem: Problem = serde_json::from_str(&problem_json)
        .map_err(|e| Error::from_reason(format!("problem json: {e}")))?;
    pm_types::normalize_problem(&mut problem);
    let solver: SolverId = serde_json::from_str(&solver_json)
        .map_err(|e| Error::from_reason(format!("solver json: {e}")))?;
    let checkpoint = match checkpoint_json {
        Some(json) => Some(
            Checkpoint::from_json(&json)
                .map_err(|e| Error::from_reason(format!("checkpoint json: {e}")))?,
        ),
        None => None,
    };
    let result = solve_resume(
        &problem,
        solver,
        SolveResume {
            checkpoint,
            iter_budget,
        },
    )
    .map_err(|e| Error::from_reason(format!("{e}")))?;
    serde_json::to_string(&result).map_err(|e| Error::from_reason(format!("{e}")))
}

#[napi]
pub fn render_svg_json(problem_json: String, solution_json: String) -> Result<String> {
    let problem: Problem = serde_json::from_str(&problem_json)
        .map_err(|e| Error::from_reason(format!("problem json: {e}")))?;
    let solution = serde_json::from_str(&solution_json)
        .map_err(|e| Error::from_reason(format!("solution json: {e}")))?;
    Ok(render(&problem, &solution))
}
