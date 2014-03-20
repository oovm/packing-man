use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ItemModel {
    IdenticalRect { width: f64, height: f64, count: u32 },
    Circle { radius: f64, count: u32 },
    Sphere { radius: f64, count: u32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemSpec {
    pub model: ItemModel,
}

impl ItemSpec {
    pub fn equal_circles(radius: f64, count: u32) -> Self {
        Self {
            model: ItemModel::Circle { radius, count },
        }
    }
}
