use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Objective {
    MaxCount,
    MinContainerSize,
    MinBins,
    MaxValue,
    Feasibility,
    MaxEqualRadius,
    MaxRadiusSum,
    MaxMinDistance,
    MaxTotalPerimeter,
    MaxVolumeUtilization,
    MinTransportCost,
}
