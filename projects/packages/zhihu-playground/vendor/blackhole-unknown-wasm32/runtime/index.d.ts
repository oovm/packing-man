/**
 * @ton/blackhole — TON 黑洞 Wasm + UCF L2/L0 呈现（宿主 WebGPU 胶水，无 Rust `wgpu` crate）。
 */
export { tonWasmRustTarget } from "./load.js";
export { loadTonWasm, renderBlackholeFrame, presentRgba8ToCanvas2d, initWebGpuHost, UcfHostHandleRegistry, } from "./present.js";
export type { LoadTonWasmOptions, TonWasmBindings, TonWasmInfo, RenderBlackholeOptions, RenderBlackholeResult, PresentTier, } from "./present.js";
//# sourceMappingURL=index.d.ts.map