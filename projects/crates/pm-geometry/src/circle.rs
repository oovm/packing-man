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
            let dx = p.cx;
            let dy = p.cy;
            (dx * dx + dy * dy).sqrt() + p.radius <= *radius + 1e-9
        }
        ContainerModel::Rectangle { width, height } => {
            p.cx - p.radius >= 0.0
                && p.cx + p.radius <= *width
                && p.cy - p.radius >= 0.0
                && p.cy + p.radius <= *height
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

pub fn container_area(container: &ContainerModel) -> f64 {
    match container {
        ContainerModel::Circle { radius } => std::f64::consts::PI * radius * radius,
        ContainerModel::Rectangle { width, height } => width * height,
        ContainerModel::Strip { width } => *width * *width,
        ContainerModel::Cube { side } => side * side * side,
        ContainerModel::Sphere { radius } => (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3),
    }
}

pub fn packed_circle_area(placements: &[Placement2d]) -> f64 {
    placements.iter().map(|p| std::f64::consts::PI * p.radius * p.radius).sum()
}

pub fn density(placements: &[Placement2d], container: &ContainerModel) -> f64 {
    let area = container_area(container);
    if area <= 0.0 {
        return 0.0;
    }
    packed_circle_area(placements) / area
}
