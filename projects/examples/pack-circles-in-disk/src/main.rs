//! Packomania 风格：等圆装入圆形容器。

use std::fs;

use pm_solver::solve;
use pm_svg::render;
use pm_types::{Problem, SolverId};

fn main() {
    let problem = Problem::circles_in_disk(1.0, 10.0, 19);
    let solution = solve(&problem, SolverId::CPU_FORCE).expect("solve");
    let svg = render(&problem, &solution);
    fs::write("pack-circles-in-disk.svg", svg).expect("write svg");
    println!(
        "pack-circles-in-disk.svg — density {:.2}% feasible={}",
        solution.metrics.density * 100.0,
        solution.feasible
    );
}
