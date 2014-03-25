//! Solution → SVG。

use pm_types::{ContainerModel, Objective, Problem, Solution};

pub fn render(problem: &Problem, solution: &Solution) -> String {
    if !solution.rect_placements.is_empty() {
        return render_strip(problem, solution);
    }
    let (view_w, view_h, transform) = view_box(problem);
    let mut out = String::new();
    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="{}" height="{}">"#,
        0.0, 0.0, view_w, view_h, view_w, view_h
    ));
    out.push('\n');
    out.push_str(&container_svg(problem, &transform));
    for p in &solution.placements {
        let (cx, cy) = transform_point(p.cx, p.cy, &transform);
        let r = p.radius * transform.scale;
        let hue = (p.id * 47 % 360) as f64;
        out.push_str(&format!(
            "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"hsl({:.0},65%,55%)\" fill-opacity=\"0.85\" stroke=\"#222\" stroke-width=\"1\"/>",
            cx, cy, r, hue
        ));
        out.push('\n');
    }
    let extra = metrics_label(problem, solution);
    out.push_str(&format!(
        "<text x=\"8\" y=\"18\" font-size=\"14\" fill=\"#333\">count={} density={:.3}% {}{}</text>",
        solution.metrics.count,
        solution.metrics.density * 100.0,
        if solution.feasible { "feasible" } else { "infeasible" },
        extra
    ));
    out.push('\n');
    out.push_str("</svg>");
    out
}

fn metrics_label(problem: &Problem, solution: &Solution) -> String {
    let mut parts = Vec::new();
    if solution.metrics.radius_sum > 0.0
        || problem.objective == Objective::MaxRadiusSum
    {
        parts.push(format!(" sum_r={:.6}", solution.metrics.radius_sum));
    }
    if solution.metrics.min_pairwise_distance > 0.0 {
        parts.push(format!(" d_min={:.6}", solution.metrics.min_pairwise_distance));
    }
    if solution.metrics.bin_count > 0 {
        parts.push(format!(" bins={}", solution.metrics.bin_count));
    }
    if solution.metrics.strip_height > 0.0 {
        parts.push(format!(" h={:.3}", solution.metrics.strip_height));
    }
    parts.join("")
}

fn render_strip(problem: &Problem, solution: &Solution) -> String {
    let strip_w = match &problem.container {
        ContainerModel::Strip { width } => *width,
        ContainerModel::Rectangle { width, .. } => *width,
        _ => 100.0,
    };
    let h = solution.metrics.strip_height.max(1.0);
    let scale = 400.0 / strip_w.max(h);
    let mut out = String::new();
    out.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {:.0} {:.0}" width="{:.0}" height="{:.0}">"#,
        strip_w * scale + 40.0,
        h * scale + 40.0,
        strip_w * scale + 40.0,
        h * scale + 40.0
    ));
    out.push('\n');
    out.push_str(&format!(
        "<rect x=\"20\" y=\"20\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"#444\" stroke-width=\"2\"/>",
        strip_w * scale,
        h * scale
    ));
    for p in &solution.rect_placements {
        out.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"hsl({},65%,60%)\" stroke=\"#222\"/>",
            20.0 + p.x * scale,
            20.0 + p.y * scale,
            p.width * scale,
            p.height * scale,
            (p.id * 53 % 360)
        ));
        out.push('\n');
    }
    out.push_str(&format!(
        "<text x=\"24\" y=\"16\" font-size=\"12\">strip_h={:.3}</text>",
        solution.metrics.strip_height
    ));
    out.push_str("</svg>");
    out
}

struct Transform {
    offset_x: f64,
    offset_y: f64,
    scale: f64,
}

fn view_box(problem: &Problem) -> (f64, f64, Transform) {
    const PAD: f64 = 20.0;
    const SIZE: f64 = 400.0;
    match &problem.container {
        ContainerModel::Circle { radius } => {
            let d = radius * 2.0;
            let scale = (SIZE - 2.0 * PAD) / d;
            (
                SIZE,
                SIZE,
                Transform {
                    offset_x: SIZE / 2.0,
                    offset_y: SIZE / 2.0,
                    scale,
                },
            )
        }
        ContainerModel::Rectangle { width, height } => {
            let scale = (SIZE - 2.0 * PAD) / width.max(*height);
            (
                width * scale + 2.0 * PAD,
                height * scale + 2.0 * PAD,
                Transform {
                    offset_x: PAD,
                    offset_y: PAD,
                    scale,
                },
            )
        }
        _ => (
            SIZE,
            SIZE,
            Transform {
                offset_x: SIZE / 2.0,
                offset_y: SIZE / 2.0,
                scale: 1.0,
            },
        ),
    }
}

fn transform_point(x: f64, y: f64, t: &Transform) -> (f64, f64) {
    (t.offset_x + x * t.scale, t.offset_y + y * t.scale)
}

fn container_svg(problem: &Problem, t: &Transform) -> String {
    match &problem.container {
        ContainerModel::Circle { radius } => {
            let r = radius * t.scale;
            format!(
                "<circle cx=\"{:.2}\" cy=\"{:.2}\" r=\"{:.2}\" fill=\"none\" stroke=\"#444\" stroke-width=\"2\"/>",
                t.offset_x, t.offset_y, r
            )
        }
        ContainerModel::Rectangle { width, height } => {
            format!(
                "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"none\" stroke=\"#444\" stroke-width=\"2\"/>",
                t.offset_x,
                t.offset_y,
                width * t.scale,
                height * t.scale
            )
        }
        _ => String::new(),
    }
}
