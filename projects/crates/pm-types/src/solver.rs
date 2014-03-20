use serde::{Deserialize, Serialize};

use crate::family::ProblemFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Cpu,
    Gpu,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AlgorithmKind {
    GreedyInsertion,
    ForceRelaxation,
    NlpSqp,
    GpuForceRelaxation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SolverId {
    pub family: ProblemFamily,
    pub backend: Backend,
    pub algorithm: AlgorithmKind,
}

impl SolverId {
    pub const CPU_GREEDY: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::GreedyInsertion,
    };

    pub const CPU_FORCE: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::ForceRelaxation,
    };

    pub const GPU_FORCE: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Gpu,
        algorithm: AlgorithmKind::GpuForceRelaxation,
    };
}
