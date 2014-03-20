use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemArchetype {
    EqualCirclesInCircle,
    EqualCirclesInSquare,
    EqualCirclesInRectangle,
    EqualCirclesInEllipse,
    VariableCirclesMaxSum,
    VariableCirclesInPerimRect,
    TammesPointsInDisk,
    CongruentCirclesCountBound,
    MaxPerimeterCirclesInSquare,
    OneDimBinPacking,
    TwoDimStripPacking,
    ThreeDimBinPacking,
    CubesInCubes,
    TetrahedraInCubes,
    OctahedraInCubes,
}
