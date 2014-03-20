import { type LoadTonWasmOptions, type TonWasmBindings } from "./load.js";
/** UCF Web 呈现档位：L2 = WebGPU 宿主提交，L0 = CPU readback。 */
export type PresentTier = "L2" | "L0";
export interface RenderBlackholeOptions extends LoadTonWasmOptions {
    mass?: number;
    /** RGBA8 写入 wasm 线性内存偏移；默认 65536。 */
    outPtr?: number;
    /** 强制 L0（调试）；默认 WebGPU 可用时走 L2。 */
    forceL0?: boolean;
}
export interface RenderBlackholeResult {
    ton: TonWasmBindings;
    tier: PresentTier;
}
export declare function presentRgba8ToCanvas2d(ctx: CanvasRenderingContext2D, memory: WebAssembly.Memory, outPtr: number, width: number, height: number): void;
/**
 * 渲染史瓦西黑洞一帧并上屏。
 * CPU 测地线在 `ton-simulation` / `ton-renderer`；上屏优先 UCF L2 WebGPU（`ton_l2_present_rgba8_webgpu` + `ucf_host`），降级 L0 readback。
 */
export declare function renderBlackholeFrame(canvas: HTMLCanvasElement, options?: RenderBlackholeOptions): Promise<RenderBlackholeResult>;
export { loadTonWasm, type LoadTonWasmOptions, type TonWasmBindings, type TonWasmInfo } from "./load.js";
export { initWebGpuHost, UcfHostHandleRegistry } from "./ucf/index.js";
//# sourceMappingURL=present.d.ts.map