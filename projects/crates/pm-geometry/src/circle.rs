use pm_types::{ContainerModel, Placement2d};

pub fn circles_overlap(a: &Placement2d, b: &Placement2d) -> bool {
    let dx = a.cx - b.cx;
    let dy = a.cy - b.cy;
    let dist_sq = dx * dx + dy * dy;
    let min_dist = a.radius + b.radius;
    dist_sq < min_dist * min_dist - 1e-12
}

pub fn circle_in_container(p: &Placement2d, container: &ContainerModel) -> bool {
    match container {
        ContainerModel::Circle { radius } => {
            let dist = (p.cx * p.cx + p.cy * p.cy).sqrt();
            dist + p.radius <= *radius + 1e-9
        }
        ContainerModel::Rectangle { width, height } => {
            p.cx - p.radius >= -1e-9
                && p.cx + p.radius <= *width + 1e-9
                && p.cy - p.radius >= -1e-9
                && p.cy + p.radius <= *height + 1e-9
        }
        ContainerModel::PerimeterBoundedRectangle { perimeter } => {
            let aspect = 1.0;
            let w = perimeter / (2.0 * (1.0 + aspect));
            let h = w * aspect;
            p.cx - p.radius >= -1e-9
                && p.cx + p.radius <= w + 1e-9
                && p.cy - p.radius >= -1e-9
                && p.cy + p.radius <= h + 1e-9
        }
        _ => false,
    }
}

pub fn all_feasible(placements: &[Placement2d], container: &ContainerModel) -> bool {
    placements.iter().all(|p| circle_in_container(p, container))
        && !pairwise_overlap(placements)
}

pub fn pairwise_overlap(placements: &[Placement2d]) -> bool {
    for i in 0..placements.len() {
        for j in (i + 1)..placements.len() {
            if circles_overlap(&placements[i], &placements[j]) {
                return true;
            }
        }
    }
    false
}

pub fn min_pairwise_distance(placements: &[Placement2d]) -> f64 {
    let mut min_d = f64::INFINITY;
    for i in 0..placements.len() {
        for j in (i + 1)..placements.len() {
            let dx = placements[i].cx - placements[j].cx;
            let dy = placements[i].cy - placements[j].cy;
            let center_dist = (dx * dx + dy * dy).sqrt();
            let surface_dist = center_dist - placements[i].radius - placements[j].radius;
            if surface_dist < min_d {
                min_d = surface_dist;
            }
        }
    }
    if min_d.is_finite() { min_d } else { 0.0 }
}

pub fn min_center_distance(placements: &[Placement2d]) -> f64 {
    let mut min_d = f64::INFINITY;
    for i in 0..placements.len() {
        for j in (i + 1)..placements.len() {
            let dx = placements[i].cx - placements[j].cx;
            let dy = placements[i].cy - placements[j].cy;
            let d = (dx * dx + dy * dy).sqrt();
            if d < min_d {
                min_d = d;
            }
        }
    }
    if min_d.is_finite() { min_d } else { 0.0 }
}

pub fn radius_sum(placements: &[Placement2d]) -> f64 {
    placements.iter().map(|p| p.radius).sum()
}

pub fn container_area(container: &ContainerModel) -> f64 {
    match container {
        ContainerModel::Circle { radius } => std::f64::consts::PI * radius * radius,
        ContainerModel::Rectangle { width, height } => width * height,
        ContainerModel::Strip { width } => *width * *width,
        ContainerModel::PerimeterBoundedRectangle { perimeter } => {
            let aspect = 1.0;
            let w = perimeter / (2.0 * (1.0 + aspect));
            w * w * aspect
        }
        ContainerModel::Cube { side } => side * side * side,
        ContainerModel::Sphere { radius } => (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3),
        ContainerModel::AxisAlignedBox { length, width, height, .. } => length * width * height,
        _ => 0.0,
    }
}

pub fn packed_circle_area(placements: &[Placement2d]) -> f64 {
    placements
        .iter()
        .map(|p| std::f64::consts::PI * p.radius * p.radius)
        .sum()
}

pub fn density(placements: &[Placement2d], container: &ContainerModel) -> f64 {
    let area = container_area(container);
    if area <= 0.0 {
        return 0.0;
    }
    packed_circle_area(placements) / area
}

/// Srivastav concentric ring count upper bound for congruent circles in a circle.
pub fn concentric_ring_count(container_r: f64, item_r: f64) -> u32 {
    if item_r <= 0.0 || container_r < item_r {
        return 0;
    }
    let mut total = 0u32;
    let mut m = 1u32;
    loop {
        let outer_r = container_r - 2.0 * (m as f64 - 1.0) * item_r;
        if outer_r < item_r {
            break;
        }
        let denom = outer_r - item_r;
        if denom <= 0.0 {
            break;
        }
        let sin_theta = item_r / denom;
        if sin_theta >= 1.0 {
            break;
        }
        let theta = sin_theta.asin();
        let n = (std::f64::consts::PI / theta).floor() as u32;
        total += n;
        m += 1;
        if m > 1000 {
            break;
        }
    }
    let inner_outer = container_r - 2.0 * (m as f64 - 1.0) * item_r;
    if inner_outer > item_r && inner_outer < 2.0 * item_r {
        total += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn srivastav_r34_r5() {
        let n = concentric_ring_count(34.0, 5.0);
        assert!(n >= 30 && n <= 38);
    }
}
