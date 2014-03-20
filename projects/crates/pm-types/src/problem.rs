use serde::{Deserialize, Serialize};

use crate::container::ContainerModel;
use crate::family::ProblemFamily;
use crate::item::ItemSpec;
use crate::objective::Objective;
use crate::rules::PlacementRuleSet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    pub family: ProblemFamily,
    pub objective: Objective,
    pub items: ItemSpec,
    pub container: ContainerModel,
    pub rules: PlacementRuleSet,
}

impl Problem {
    pub fn circles_in_disk(radius: f64, container_r: f64, count: u32) -> Self {
        Self {
            family: ProblemFamily::CircleSpherePacking,
            objective: Objective::MaxCount,
            items: ItemSpec::equal_circles(radius, count),
            container: ContainerModel::Circle { radius: container_r },
            rules: PlacementRuleSet::default(),
        }
    }
}
