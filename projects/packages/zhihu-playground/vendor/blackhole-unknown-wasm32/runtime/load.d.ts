import { UcfHostHandleRegistry } from "./ucf/index.js";
export declare const tonWasmRustTarget: "wasm32-unknown-unknown";
export interface TonWasmInfo {
    name: string;
    version: string;
    versionCode: number;
    targetTriple: string;
    ucfDiagMask: number;
}
export interface TonWasmBindings {
    info(): TonWasmInfo;
    renderSchwarzschildRgba8(width: number, height: number, mass: number, outPtr: number): number;
    l2PresentRgba8Webgpu(width: number, height: number, ptr: number, byteLen: number): number;
    ucfL2RasterTriWebgpu(width: number, height: number): number;
    memory: WebAssembly.Memory;
}
export interface LoadTonWasmOptions {
    wasmUrl?: string | URL;
    module?: WebAssembly.Module;
    registry?: UcfHostHandleRegistry;
}
export declare function loadTonWasm(options?: LoadTonWasmOptions): Promise<TonWasmBindings>;
//# sourceMappingURL=load.d.ts.map