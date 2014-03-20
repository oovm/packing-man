use thiserror::Error;

use crate::family::ProblemFamily;

pub type PmResult<T> = Result<T, PmError>;

#[derive(Debug, Error)]
pub enum PmError {
    #[error("unsupported problem family: {0:?}")]
    UnsupportedFamily(ProblemFamily),
    #[error("unsupported solver for family {family:?}: {algorithm}")]
    UnsupportedSolver {
        family: ProblemFamily,
        algorithm: String,
    },
    #[error("invalid problem: {0}")]
    InvalidProblem(String),
    #[error("gpu unavailable")]
    GpuUnavailable,
    #[error("serialization: {0}")]
    Serde(String),
}
