use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VehicleSpec {
    pub id: u32,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub max_weight_kg: f64,
    pub cost_per_trip: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContainerModel {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Strip { width: f64 },
    Cube { side: f64 },
    Sphere { radius: f64 },
    PerimeterBoundedRectangle { perimeter: f64 },
    AxisAlignedBox {
        length: f64,
        width: f64,
        height: f64,
        max_weight_kg: f64,
    },
    VehicleFleet { vehicles: Vec<VehicleSpec> },
}

impl ContainerModel {
    pub fn unit_square() -> Self {
        Self::Rectangle {
            width: 1.0,
            height: 1.0,
        }
    }

    pub fn perim_rect_from_perimeter(perimeter: f64, aspect: f64) -> Self {
        let w = perimeter / (2.0 * (1.0 + aspect));
        Self::Rectangle {
            width: w,
            height: w * aspect,
        }
    }
}
