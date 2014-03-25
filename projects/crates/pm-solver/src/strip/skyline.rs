use pm_geometry::orthogonal::all_rects_feasible;
use pm_types::{ContainerModel, ItemModel, Problem, ProblemFamily};

use crate::common::build_strip_solution;

#[derive(Clone)]
struct SkylineSeg {
    x: f64,
    y: f64,
    width: f64,
}

pub fn solve(problem: &Problem) -> Option<pm_types::Solution> {
    if problem.family != ProblemFamily::ManufacturerPalletLoading {
        return None;
    }
    let (items, strip_width) = match (&problem.items.model, &problem.container) {
        (ItemModel::OrthogonalBoxes { items }, ContainerModel::Strip { width }) => (items, *width),
        (
            ItemModel::OrthogonalBoxes { items },
            ContainerModel::Rectangle { width, .. },
        ) => (items, *width),
        _ => return None,
    };
    if items.is_empty() {
        return Some(build_strip_solution(problem, vec![], true, "skyline_strip"));
    }

    let mut sorted = items.clone();
    sorted.sort_by(|a, b| {
        (b.length * b.height)
            .partial_cmp(&(a.length * a.height))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut skyline = vec![SkylineSeg {
        x: 0.0,
        y: 0.0,
        width: strip_width,
    }];
    let mut placements = Vec::new();

    for item in &sorted {
        let (w, h) = (item.length, item.height);
        let mut best: Option<(f64, f64, usize)> = None;
        for (si, seg) in skyline.iter().enumerate() {
            if seg.width < w {
                continue;
            }
            let x = seg.x;
            let y = seg.y;
            let score = y * 1000.0 + x;
            if best.map(|(s, _, _)| score < s).unwrap_or(true) {
                best = Some((score, y, si));
            }
        }
        if let Some((_, y, si)) = best {
            let x = skyline[si].x;
            placements.push(pm_types::PlacementRect {
                id: item.id,
                x,
                y,
                width: w,
                height: h,
            });
            let new_y = y + h;
            update_skyline(&mut skyline, si, x, w, new_y, strip_width);
        }
    }

    let feasible = all_rects_feasible(&placements, strip_width);
    Some(build_strip_solution(problem, placements, feasible, "skyline_strip"))
}

fn update_skyline(
    skyline: &mut Vec<SkylineSeg>,
    idx: usize,
    _x: f64,
    w: f64,
    new_y: f64,
    strip_width: f64,
) {
    let seg = skyline[idx].clone();
    let left = SkylineSeg {
        x: seg.x,
        y: new_y,
        width: w,
    };
    let right_x = seg.x + w;
    let right_w = seg.width - w;
    skyline.remove(idx);
    if right_w > 1e-9 {
        skyline.insert(
            idx,
            SkylineSeg {
                x: right_x,
                y: seg.y,
                width: right_w,
            },
        );
    }
    skyline.insert(idx, left);
    merge_skyline(skyline, strip_width);
}

fn merge_skyline(skyline: &mut Vec<SkylineSeg>, strip_width: f64) {
    skyline.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap_or(std::cmp::Ordering::Equal));
    let mut i = 0;
    while i + 1 < skyline.len() {
        if skyline[i].y == skyline[i + 1].y {
            skyline[i].width += skyline[i + 1].width;
            skyline.remove(i + 1);
        } else {
            i += 1;
        }
    }
    if skyline.is_empty() {
        skyline.push(SkylineSeg {
            x: 0.0,
            y: 0.0,
            width: strip_width,
        });
    }
}
