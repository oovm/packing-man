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
    AnalyticalConcentricRing,
    NlpLocalSearch,
    EvolutionaryConstruction,
    FirstFitDecreasing,
    BestFitDecreasing,
    KarmarkarKarpGrouping,
    SkylineStrip,
    ExtremePointBlf,
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

    pub const CPU_CONCENTRIC: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::AnalyticalConcentricRing,
    };

    pub const CPU_NLP: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::NlpLocalSearch,
    };

    pub const CPU_FFD: Self = Self {
        family: ProblemFamily::ManufacturerPalletLoading,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::FirstFitDecreasing,
    };

    pub const CPU_BFD: Self = Self {
        family: ProblemFamily::ManufacturerPalletLoading,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::BestFitDecreasing,
    };

    pub const CPU_SKYLINE: Self = Self {
        family: ProblemFamily::ManufacturerPalletLoading,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::SkylineStrip,
    };

    pub const CPU_EXTREME_POINT: Self = Self {
        family: ProblemFamily::ManufacturerPalletLoading,
        backend: Backend::Cpu,
        algorithm: AlgorithmKind::ExtremePointBlf,
    };

    pub const GPU_FORCE: Self = Self {
        family: ProblemFamily::CircleSpherePacking,
        backend: Backend::Gpu,
        algorithm: AlgorithmKind::GpuForceRelaxation,
    };
}
