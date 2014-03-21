//! 三维轴对齐箱体几何。

use pm_types::{ContainerModel, Placement3d};

pub fn boxes_overlap(a: &Placement3d, b: &Placement3d) -> bool {
    a.x < b.x + b.dx
        && a.x + a.dx > b.x
        && a.y < b.y + b.dy
        && a.y + a.dy > b.y
        && a.z < b.z + b.dz
        && a.z + a.dz > b.z
}

pub fn box_in_container(p: &Placement3d, container: &ContainerModel) -> bool {
    match container {
        ContainerModel::Cube { side } => {
            p.x >= 0.0
                && p.y >= 0.0
                && p.z >= 0.0
                && p.x + p.dx <= *side
                && p.y + p.dy <= *side
                && p.z + p.dz <= *side
        }
        ContainerModel::AxisAlignedBox {
            length,
            width,
            height,
            ..
        } => {
            p.x >= 0.0
                && p.y >= 0.0
                && p.z >= 0.0
                && p.x + p.dx <= *length
                && p.y + p.dy <= *width
                && p.z + p.dz <= *height
        }
        _ => false,
    }
}

pub fn all_boxes_feasible(placements: &[Placement3d], container: &ContainerModel) -> bool {
    placements.iter().all(|p| box_in_container(p, container))
        && !pairwise_box_overlap(placements)
}

pub fn pairwise_box_overlap(placements: &[Placement3d]) -> bool {
    for i in 0..placements.len() {
        for j in (i + 1)..placements.len() {
            if boxes_overlap(&placements[i], &placements[j]) {
                return true;
            }
        }
    }
    false
}

pub fn volume_utilization(placements: &[Placement3d], container: &ContainerModel) -> f64 {
    let container_vol = match container {
        ContainerModel::Cube { side } => side.powi(3),
        ContainerModel::AxisAlignedBox { length, width, height, .. } => length * width * height,
        _ => 0.0,
    };
    if container_vol <= 0.0 {
        return 0.0;
    }
    let packed = placements.iter().map(|p| p.dx * p.dy * p.dz).sum::<f64>();
    packed / container_vol
}

pub fn center_of_gravity_3d(placements: &[Placement3d]) -> (f64, f64, f64) {
    let mut sx = 0.0;
    let mut sy = 0.0;
    let mut sz = 0.0;
    let mut w = 0.0;
    for p in placements {
        let vol = p.dx * p.dy * p.dz;
        sx += (p.x + p.dx / 2.0) * vol;
        sy += (p.y + p.dy / 2.0) * vol;
        sz += (p.z + p.dz / 2.0) * vol;
        w += vol;
    }
    if w > 0.0 {
        (sx / w, sy / w, sz / w)
    } else {
        (0.0, 0.0, 0.0)
    }
}

pub fn axle_load_ok(
    placements: &[Placement3d],
    container_length: f64,
    front_max: f64,
    rear_max: f64,
) -> bool {
    let mid = container_length / 2.0;
    let mut front = 0.0;
    let mut rear = 0.0;
    for p in placements {
        let vol = p.dx * p.dy * p.dz;
        let cx = p.x + p.dx / 2.0;
        if cx <= mid {
            front += vol;
        } else {
            rear += vol;
        }
    }
    front <= front_max && rear <= rear_max
}
