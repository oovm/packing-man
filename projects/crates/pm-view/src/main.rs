//! Packing-Man CLI：求解并输出 SVG。

use std::env;
use std::fs;
use std::process::ExitCode;

use pm_solver::solve;
use pm_svg::render;
use pm_types::{Problem, SolverId};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let out = args.get(1).map(String::as_str).unwrap_or("packing.svg");
    let container_r = parse_f64(&args, "--container-r", 10.0);
    let circle_r = parse_f64(&args, "--circle-r", 1.0);
    let count = parse_u32(&args, "--count", 19);
    let algorithm = args
        .iter()
        .position(|a| a == "--algorithm")
        .and_then(|i| args.get(i + 1))
        .map(String::as_str)
        .unwrap_or("force");

    let problem = Problem::circles_in_disk(circle_r, container_r, count);
    let solver = match algorithm {
        "greedy" => SolverId::CPU_GREEDY,
        "gpu" => SolverId::GPU_FORCE,
        _ => SolverId::CPU_FORCE,
    };

    match solve(&problem, solver) {
        Ok(solution) => {
            let svg = render(&problem, &solution);
            if let Err(e) = fs::write(out, svg) {
                eprintln!("write failed: {e}");
                return ExitCode::from(1);
            }
            eprintln!(
                "wrote {} — count={} density={:.2}% feasible={}",
                out,
                solution.metrics.count,
                solution.metrics.density * 100.0,
                solution.feasible
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("solve failed: {e}");
            ExitCode::from(2)
        }
    }
}

fn parse_f64(args: &[String], flag: &str, default: f64) -> f64 {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn parse_u32(args: &[String], flag: &str, default: u32) -> u32 {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
