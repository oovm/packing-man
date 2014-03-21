//! 正交矩形族几何。

use pm_types::{PlacementRect, Solution};

pub fn rects_overlap(a: &PlacementRect, b: &PlacementRect) -> bool {
    a.x < b.x + b.width
        && a.x + a.width > b.x
        && a.y < b.y + b.height
        && a.y + a.height > b.y
}

pub fn rect_in_strip(p: &PlacementRect, strip_width: f64) -> bool {
    p.x >= 0.0 && p.x + p.width <= strip_width && p.y >= 0.0
}

pub fn strip_height(placements: &[PlacementRect]) -> f64 {
    placements
        .iter()
        .map(|p| p.y + p.height)
        .fold(0.0, f64::max)
}

pub fn all_rects_feasible(placements: &[PlacementRect], strip_width: f64) -> bool {
    placements.iter().all(|p| rect_in_strip(p, strip_width))
        && !pairwise_rect_overlap(placements)
}

pub fn pairwise_rect_overlap(placements: &[PlacementRect]) -> bool {
    for i in 0..placements.len() {
        for j in (i + 1)..placements.len() {
            if rects_overlap(&placements[i], &placements[j]) {
                return true;
            }
        }
    }
    false
}

pub fn center_of_gravity_2d(solution: &Solution) -> (f64, f64) {
    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut w = 0.0;
    for p in &solution.rect_placements {
        let area = p.width * p.height;
        sum_x += (p.x + p.width / 2.0) * area;
        sum_y += (p.y + p.height / 2.0) * area;
        w += area;
    }
    if w > 0.0 {
        (sum_x / w, sum_y / w)
    } else {
        (0.0, 0.0)
    }
}
