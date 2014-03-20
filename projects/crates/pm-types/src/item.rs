use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BoxItemKind {
    #[default]
    Standard,
    Fragile,
    Oriented,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxSpec {
    pub id: u32,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub weight_kg: f64,
    #[serde(default)]
    pub kind: BoxItemKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ItemModel {
    Circle { radius: f64, count: u32 },
    IdenticalCircles { count: u32, radius: Option<f64> },
    VariableCircles { count: u32 },
    Sphere { radius: f64, count: u32 },
    IdenticalRect { width: f64, height: f64, count: u32 },
    OneDimItems { sizes: Vec<f64> },
    OrthogonalBoxes { items: Vec<BoxSpec> },
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

    pub fn variable_circles(count: u32) -> Self {
        Self {
            model: ItemModel::VariableCircles { count },
        }
    }

    pub fn identical_circles(count: u32, radius: Option<f64>) -> Self {
        Self {
            model: ItemModel::IdenticalCircles { count, radius },
        }
    }

    pub fn one_dim(sizes: Vec<f64>) -> Self {
        Self {
            model: ItemModel::OneDimItems { sizes },
        }
    }

    pub fn orthogonal_cubes(side: f64, count: u32) -> Self {
        let items = (0..count)
            .map(|id| BoxSpec {
                id,
                length: side,
                width: side,
                height: side,
                weight_kg: 1.0,
                kind: BoxItemKind::Standard,
            })
            .collect();
        Self {
            model: ItemModel::OrthogonalBoxes { items },
        }
    }

    pub fn circle_count(&self) -> Option<u32> {
        match &self.model {
            ItemModel::Circle { count, .. } => Some(*count),
            ItemModel::IdenticalCircles { count, .. } => Some(*count),
            ItemModel::VariableCircles { count } => Some(*count),
            _ => None,
        }
    }

    pub fn fixed_circle_radius(&self) -> Option<f64> {
        match &self.model {
            ItemModel::Circle { radius, .. } => Some(*radius),
            ItemModel::IdenticalCircles { radius: Some(r), .. } => Some(*r),
            _ => None,
        }
    }
}
