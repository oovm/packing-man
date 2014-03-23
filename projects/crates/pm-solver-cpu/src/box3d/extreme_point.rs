use pm_geometry::box3d::{all_boxes_feasible, volume_utilization};
use pm_types::{
    ContainerModel, ItemModel, Placement3d, Problem, ProblemFamily, Solution, SolutionMetrics,
    SolveMeta,
};

#[derive(Clone, Copy)]
struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

pub fn solve(problem: &Problem) -> Option<Solution> {
    if problem.family != ProblemFamily::ManufacturerPalletLoading {
        return None;
    }
    let (box_specs, container) = match (&problem.items.model, &problem.container) {
        (
            ItemModel::OrthogonalBoxes { items },
            ContainerModel::AxisAlignedBox { .. },
        ) => (items.clone(), problem.container.clone()),
        (
            ItemModel::OrthogonalBoxes { items },
            ContainerModel::Cube { side },
        ) => (
            items.clone(),
            ContainerModel::AxisAlignedBox {
                length: *side,
                width: *side,
                height: *side,
                max_weight_kg: f64::MAX,
            },
        ),
        _ => return None,
    };

    let (cl, cw, ch) = box_dims(&container);
    let mut sorted = box_specs;
    sorted.sort_by(|a, b| {
        (b.length * b.width * b.height)
            .partial_cmp(&(a.length * a.width * a.height))
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut placements = Vec::new();
    let mut extreme_points = vec![Point3 { x: 0.0, y: 0.0, z: 0.0 }];

    for item in &sorted {
        let mut placed = false;
        extreme_points.sort_by(|a, b| {
            (a.z, a.y, a.x)
                .partial_cmp(&(b.z, b.y, b.x))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for ep in extreme_points.clone() {
            if ep.x + item.length <= cl && ep.y + item.width <= cw && ep.z + item.height <= ch {
                let candidate = Placement3d {
                    id: item.id,
                    x: ep.x,
                    y: ep.y,
                    z: ep.z,
                    dx: item.length,
                    dy: item.width,
                    dz: item.height,
                    bin_id: 0,
                };
                if placements
                    .iter()
                    .all(|p| !pm_geometry::box3d::boxes_overlap(p, &candidate))
                {
                    add_extreme_points(&mut extreme_points, &candidate, cl, cw, ch);
                    placements.push(candidate);
                    placed = true;
                    break;
                }
            }
        }
        if !placed {
            break;
        }
    }

    let feasible = all_boxes_feasible(&placements, &container);
    let util = volume_utilization(&placements, &container);
    let count = placements.len() as u32;
    Some(Solution {
        family: problem.family,
        placements: vec![],
        rect_placements: vec![],
        placements_3d: placements,
        bins_1d: vec![],
        bins: vec![],
        metrics: SolutionMetrics {
            count,
            volume_utilization: util,
            objective_value: util,
            ..SolutionMetrics::default()
        },
        feasible,
        meta: SolveMeta {
            algorithm: "extreme_point_blf".to_string(),
            backend: "cpu".to_string(),
            iterations: 0,
            elapsed_ms: 0,
        },
    })
}

fn box_dims(container: &ContainerModel) -> (f64, f64, f64) {
    match container {
        ContainerModel::AxisAlignedBox {
            length,
            width,
            height,
            ..
        } => (*length, *width, *height),
        ContainerModel::Cube { side } => (*side, *side, *side),
        _ => (0.0, 0.0, 0.0),
    }
}

fn add_extreme_points(points: &mut Vec<Point3>, p: &Placement3d, cl: f64, cw: f64, ch: f64) {
    let candidates = [
        Point3 {
            x: p.x + p.dx,
            y: p.y,
            z: p.z,
        },
        Point3 {
            x: p.x,
            y: p.y + p.dy,
            z: p.z,
        },
        Point3 {
            x: p.x,
            y: p.y,
            z: p.z + p.dz,
        },
    ];
    for c in candidates {
        if c.x < cl && c.y < cw && c.z < ch {
            points.push(c);
        }
    }
    points.retain(|pt| pt.x < cl && pt.y < cw && pt.z < ch);
}
