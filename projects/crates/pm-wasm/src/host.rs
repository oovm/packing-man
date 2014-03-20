//! `pm_host` wasm import：WebGPU RGBA8 上屏（浏览器 API，非 `wgpu` crate）。

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "pm_host")]
unsafe extern "C" {
    /// 从 wasm 线性内存 `[ptr, ptr+byte_len)` 上传并绘制到当前 WebGPU swapchain。
    pub fn pm_host_present_rgba8_webgpu(
        width: u32,
        height: u32,
        ptr: u32,
        byte_len: u32,
    ) -> i32;
}

pub fn present_rgba8_webgpu_via_host(
    width: u32,
    height: u32,
    ptr: u32,
    byte_len: u32,
) -> i32 {
    #[cfg(target_arch = "wasm32")]
    unsafe {
        pm_host_present_rgba8_webgpu(width, height, ptr, byte_len)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (width, height, ptr, byte_len);
        -50
    }
}
