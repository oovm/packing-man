use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ContainerModel {
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
    Strip { width: f64 },
    Cube { side: f64 },
    Sphere { radius: f64 },
}
