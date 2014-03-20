/**
 * WebGPU 宿主：上传 wasm RGBA8 并绘制全屏四边形。
 */
export interface TonWebGpuState {
    device: GPUDevice;
    context: GPUCanvasContext;
    format: GPUTextureFormat;
    pipeline: GPURenderPipeline;
    sampler: GPUSampler;
}
export declare function initTonWebGpuHost(canvas: HTMLCanvasElement): Promise<TonWebGpuState>;
export declare function getTonWebGpuState(): TonWebGpuState | undefined;
/** wasm import `ton_host_present_rgba8_webgpu` 实现。 */
export declare function hostPresentRgba8WebGpu(width: number, height: number, memory: WebAssembly.Memory, ptr: number, byteLen: number): number;
//# sourceMappingURL=webgpu.d.ts.map