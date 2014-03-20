use serde::{Deserialize, Serialize};

/// Birgin 四族 — 架构一级分类。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemFamily {
    ManufacturerPalletLoading,
    DistributorPalletLoading,
    CircleSpherePacking,
    ConvexRegionPacking,
}
