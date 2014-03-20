//! Wasm 绑定：JSON 求解 + SVG 渲染。

use pm_solver::solve;
use pm_svg::render;
use pm_types::{Problem, SolverId};

pub const VERSION_CODE: u32 = 0_002_000;

#[unsafe(no_mangle)]
pub extern "C" fn pm_version_code() -> u32 {
    VERSION_CODE
}

fn write_bytes(out: *mut u8, cap: u32, data: &[u8]) -> i32 {
    if out.is_null() || cap < data.len() as u32 {
        return -(data.len() as i32);
    }
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), out, data.len());
    }
    data.len() as i32
}

/// JSON problem + solver discriminant in JSON: `{ "problem": ..., "solver": ... }`
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pm_solve_json(
    in_ptr: *const u8,
    in_len: u32,
    out_ptr: *mut u8,
    out_cap: u32,
) -> i32 {
    if in_ptr.is_null() || in_len == 0 {
        return -1;
    }
    let input = unsafe { std::slice::from_raw_parts(in_ptr, in_len as usize) };
    let req: serde_json::Value = match serde_json::from_slice(input) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    let problem: Problem = match serde_json::from_value(req["problem"].clone()) {
        Ok(p) => p,
        Err(_) => return -3,
    };
    let solver: SolverId = match serde_json::from_value(req["solver"].clone()) {
        Ok(s) => s,
        Err(_) => return -4,
    };
    let solution = match solve(&problem, solver) {
        Ok(s) => s,
        Err(_) => return -5,
    };
    let json = match serde_json::to_vec(&solution) {
        Ok(b) => b,
        Err(_) => return -6,
    };
    write_bytes(out_ptr, out_cap, &json)
}

/// JSON `{ "problem": ..., "solution": ... }` → SVG UTF-8
#[unsafe(no_mangle)]
pub unsafe extern "C" fn pm_render_svg(
    in_ptr: *const u8,
    in_len: u32,
    out_ptr: *mut u8,
    out_cap: u32,
) -> i32 {
    if in_ptr.is_null() || in_len == 0 {
        return -1;
    }
    let input = unsafe { std::slice::from_raw_parts(in_ptr, in_len as usize) };
    let req: serde_json::Value = match serde_json::from_slice(input) {
        Ok(v) => v,
        Err(_) => return -2,
    };
    let problem: Problem = match serde_json::from_value(req["problem"].clone()) {
        Ok(p) => p,
        Err(_) => return -3,
    };
    let solution = match serde_json::from_value(req["solution"].clone()) {
        Ok(s) => s,
        Err(_) => return -4,
    };
    let svg = render(&problem, &solution);
    write_bytes(out_ptr, out_cap, svg.as_bytes())
}
