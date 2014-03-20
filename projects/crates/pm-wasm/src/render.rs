//! 场景一帧 RGBA8 渲染（积分器 × 盘模型 × 自旋 × 后处理可切换）。

use pm_simulation::{black_hole_for_render, Scene};
use pm_types::{
    post_bloom_from_flags, use_ucf_post, DiskModelKind, IntegratorKind, TonError,
};

/// 负错误码：未知 `integrator` 判别值。
pub const ERR_INVALID_INTEGRATOR: i32 = -3;
/// 负错误码：未知 `disk_model` 判别值。
pub const ERR_INVALID_DISK_MODEL: i32 = -4;
/// 负错误码：自旋参数非法（非有限或 `|a| >= M` 于 Kerr 路径）。
pub const ERR_INVALID_SPIN: i32 = -5;

fn ton_error_to_code(err: TonError) -> i32 {
    match err {
        TonError::Domain(_) => ERR_INVALID_SPIN,
        TonError::Captured => -6,
        TonError::Escaped => -7,
    }
}

pub fn render_frame_rgba8(
    width: u32,
    height: u32,
    mass: f64,
    spin: f64,
    integrator: IntegratorKind,
    disk_model: DiskModelKind,
    post_flags: u8,
) -> Result<Vec<u8>, i32> {
    if width == 0 || height == 0 {
        return Ok(Vec::new());
    }
    let black_hole = black_hole_for_render(mass, spin, integrator).map_err(ton_error_to_code)?;
    let mut builder = Scene::builder(black_hole)
        .with_integrator(integrator)
        .with_disk_model(disk_model)
        .with_post_bloom(post_bloom_from_flags(post_flags))
        .with_resolution(width, height);
    if disk_model != DiskModelKind::None {
        builder = builder.with_default_disk();
    }
    let scene = builder.build();
    if use_ucf_post(post_flags) {
        scene
            .render_frame_ucf_cpu()
            .map(|(fb, _)| fb.to_rgba8())
            .map_err(ton_error_to_code)
    } else {
        Ok(scene.render_frame().to_rgba8())
    }
}

pub fn render_schwarzschild_rgba8(
    width: u32,
    height: u32,
    mass: f64,
) -> Result<Vec<u8>, i32> {
    render_frame_rgba8(
        width,
        height,
        mass,
        0.0,
        IntegratorKind::SchwarzschildOrbit,
        DiskModelKind::NovikovThorneThin,
        0,
    )
}

/// 写入线性内存；返回 `0` 或负错误码。
///
/// # Safety
///
/// `out` 须可写且至少 `width * height * 4` 字节。
pub unsafe fn write_frame_rgba8(
    width: u32,
    height: u32,
    mass: f64,
    spin: f64,
    integrator: u8,
    disk_model: u8,
    post_flags: u8,
    out: *mut u8,
) -> i32 {
    if out.is_null() {
        return -1;
    }
    let integrator = match IntegratorKind::from_u8(integrator) {
        Some(v) => v,
        None => return ERR_INVALID_INTEGRATOR,
    };
    let disk_model = match DiskModelKind::from_u8(disk_model) {
        Some(v) => v,
        None => return ERR_INVALID_DISK_MODEL,
    };
    let need = (width as usize).saturating_mul(height as usize).saturating_mul(4);
    if need == 0 {
        return 0;
    }
    match render_frame_rgba8(width, height, mass, spin, integrator, disk_model, post_flags) {
        Ok(bytes) if bytes.len() == need => {
            unsafe {
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), out, need);
            }
            0
        }
        Ok(_) => -2,
        Err(code) => code,
    }
}

/// # Safety
///
/// `out` 须可写且至少 `width * height * 4` 字节。
pub unsafe fn write_schwarzschild_rgba8(
    width: u32,
    height: u32,
    mass: f64,
    out: *mut u8,
) -> i32 {
    unsafe { write_frame_rgba8(width, height, mass, 0.0, 0, 1, 0, out) }
}
