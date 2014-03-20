use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolyominoKind {
    Tan,
    Domino,
    LShape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShapeKind {
    Circle,
    Square,
    Rectangle,
    EquilateralTriangle,
    RegularPolygon { sides: u8 },
    Polyomino { kind: PolyominoKind },
    Ellipse,
    ArbitraryTriangle,
    Cube,
    Tetrahedron,
    Octahedron,
    OrthogonalBox,
}

impl ShapeKind {
    pub fn is_v2_supported(&self) -> bool {
        matches!(
            self,
            ShapeKind::Circle
                | ShapeKind::Square
                | ShapeKind::Rectangle
                | ShapeKind::Cube
                | ShapeKind::OrthogonalBox
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ShapePair {
    pub item: ShapeKind,
    pub container: ShapeKind,
}

impl ShapePair {
    pub const CIRCLES_IN_CIRCLE: Self = Self {
        item: ShapeKind::Circle,
        container: ShapeKind::Circle,
    };

    pub const CIRCLES_IN_SQUARE: Self = Self {
        item: ShapeKind::Circle,
        container: ShapeKind::Square,
    };

    pub const CIRCLES_IN_RECTANGLE: Self = Self {
        item: ShapeKind::Circle,
        container: ShapeKind::Rectangle,
    };

    pub const CUBES_IN_CUBES: Self = Self {
        item: ShapeKind::Cube,
        container: ShapeKind::Cube,
    };
}

impl Default for ShapePair {
    fn default() -> Self {
        Self::CIRCLES_IN_CIRCLE
    }
}
