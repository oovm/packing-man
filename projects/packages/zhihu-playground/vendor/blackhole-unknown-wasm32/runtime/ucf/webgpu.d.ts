/**
 * UCF L2 宿主 WebGPU 桥（浏览器 API，非 Rust `wgpu` crate）。
 * WGSL 仅用于 L2 宿主 companion（与 `ucf_emitter` 薄门对齐），不是产品测地线着色器。
 */
export interface WebGpuHostState {
    device: GPUDevice;
    context: GPUCanvasContext;
    format: GPUTextureFormat;
}
export declare function initWebGpuHost(canvas: HTMLCanvasElement): Promise<WebGpuHostState>;
export declare function getWebGpuHostState(): WebGpuHostState | undefined;
export declare function hostL2ColorFillWebGpu(width: number, height: number, r: number, g: number, b: number, a: number): number;
export declare function hostL2RasterClearWebGpu(width: number, height: number, r: number, g: number, b: number, a: number): number;
export declare function hostL2RasterTriWebGpu(width: number, height: number): number;
/** wasm import `ton_host_present_rgba8_webgpu`：wasm 线性内存 RGBA8 → WebGPU swapchain。 */
export declare function hostPresentRgba8WebGpu(width: number, height: number, memory: WebAssembly.Memory, ptr: number, byteLen: number): number;
//# sourceMappingURL=webgpu.d.ts.map