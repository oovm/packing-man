//! Packing-Man CLI：求解并输出 SVG。

use std::env;
use std::fs;
use std::process::ExitCode;

use pm_benchmark::{all_paths, bound_gap, load_by_path, problem_from_fixture, solver_for_problem};
use pm_solver::solve;
use pm_svg::render;
use pm_types::{Problem, SolverId};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "--list-fixtures") {
        for path in all_paths() {
            println!("{path}");
        }
        return ExitCode::SUCCESS;
    }

    if let Some(fixture_path) = flag_value(&args, "--fixture") {
        return run_fixture(&fixture_path, &args);
    }

    let out = args.get(1).map(String::as_str).unwrap_or("packing.svg");
    let container_r = parse_f64(&args, "--container-r", 10.0);
    let circle_r = parse_f64(&args, "--circle-r", 1.0);
    let count = parse_u32(&args, "--count", 19);
    let algorithm = flag_value(&args, "--algorithm").unwrap_or_else(|| "force".to_string());

    let problem = Problem::circles_in_disk(circle_r, container_r, count);
    let solver = match algorithm.as_str() {
        "greedy" => SolverId::CPU_GREEDY,
        "concentric" => SolverId::CPU_CONCENTRIC,
        "gpu" => SolverId::GPU_FORCE,
        _ => SolverId::CPU_FORCE,
    };

    run_solve(&problem, solver, out)
}

fn run_fixture(path: &str, args: &[String]) -> ExitCode {
    let fixture = match load_by_path(path) {
        Some(f) => f,
        None => {
            eprintln!("unknown fixture: {path}");
            eprintln!("use --list-fixtures to see paths");
            return ExitCode::from(1);
        }
    };
    let problem = problem_from_fixture(&fixture);
    let out = flag_value(args, "--out").unwrap_or_else(|| "packing.svg".to_string());
    let solver = solver_for_problem(&problem);
    run_fixture_solve(&fixture, &problem, solver, &out)
}

fn run_fixture_solve(
    fixture: &pm_benchmark::Fixture,
    problem: &Problem,
    solver: SolverId,
    out: &str,
) -> ExitCode {
    match solve(problem, solver) {
        Ok(solution) => {
            let svg = render(problem, &solution);
            if let Err(e) = fs::write(out, svg) {
                eprintln!("write failed: {e}");
                return ExitCode::from(1);
            }
            let gap_msg = bound_gap(fixture, &solution)
                .map(|g| format!(" bound_gap={:.6}", g))
                .unwrap_or_default();
            eprintln!(
                "wrote {} — id={} count={} density={:.2}% radius_sum={:.6} feasible={} obj={:.6}{}",
                out,
                fixture.id,
                solution.metrics.count,
                solution.metrics.density * 100.0,
                solution.metrics.radius_sum,
                solution.feasible,
                solution.metrics.objective_value,
                gap_msg
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("solve failed: {e}");
            ExitCode::from(2)
        }
    }
}

fn run_solve(problem: &Problem, solver: SolverId, out: &str) -> ExitCode {
    match solve(problem, solver) {
        Ok(solution) => {
            let svg = render(problem, &solution);
            if let Err(e) = fs::write(out, svg) {
                eprintln!("write failed: {e}");
                return ExitCode::from(1);
            }
            eprintln!(
                "wrote {} — count={} density={:.2}% radius_sum={:.6} feasible={} obj={:.6}",
                out,
                solution.metrics.count,
                solution.metrics.density * 100.0,
                solution.metrics.radius_sum,
                solution.feasible,
                solution.metrics.objective_value
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("solve failed: {e}");
            ExitCode::from(2)
        }
    }
}

fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|i| args.get(i + 1))
        .map(String::clone)
}

fn parse_f64(args: &[String], flag: &str, default: f64) -> f64 {
    flag_value(args, flag)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}

fn parse_u32(args: &[String], flag: &str, default: u32) -> u32 {
    flag_value(args, flag)
        .and_then(|s| s.parse().ok())
        .unwrap_or(default)
}
