export { getWebGpuHostState, hostL2ColorFillWebGpu, hostL2RasterClearWebGpu, hostL2RasterTriWebGpu, hostPresentRgba8WebGpu, initWebGpuHost, } from "./webgpu.js";
export type { WebGpuHostState } from "./webgpu.js";
/** 逻辑 `resource_id` → 宿主句柄表（UCF `ucf_host_bind_external_buffer`）。 */
export declare class UcfHostHandleRegistry {
    #private;
    register(handleId: number, handle: {
        kind: string;
        native?: unknown;
    }): void;
    bindResource(resourceId: number, handleId: number): number;
}
export declare function buildUcfHostImports(registry: UcfHostHandleRegistry): WebAssembly.ModuleImports;
export declare function buildTonHostImports(memoryRef: {
    current?: WebAssembly.Memory;
}): WebAssembly.ModuleImports;
//# sourceMappingURL=index.d.ts.map