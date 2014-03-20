import { hostL2ColorFillWebGpu, hostL2RasterClearWebGpu, hostL2RasterTriWebGpu, hostPresentRgba8WebGpu, } from "./webgpu.js";
export { getWebGpuHostState, hostL2ColorFillWebGpu, hostL2RasterClearWebGpu, hostL2RasterTriWebGpu, hostPresentRgba8WebGpu, initWebGpuHost, } from "./webgpu.js";
/** 逻辑 `resource_id` → 宿主句柄表（UCF `ucf_host_bind_external_buffer`）。 */
export class UcfHostHandleRegistry {
    #handles = new Map();
    #bindings = new Map();
    register(handleId, handle) {
        this.#handles.set(handleId, handle);
    }
    bindResource(resourceId, handleId) {
        if (!this.#handles.has(handleId)) {
            return -1;
        }
        this.#bindings.set(resourceId, handleId);
        return 0;
    }
}
export function buildUcfHostImports(registry) {
    return {
        ucf_host_bind_external_buffer: (resourceId, handleId) => registry.bindResource(resourceId, handleId),
        ucf_host_l2_color_fill_webgpu: (width, height, r, g, b, a) => hostL2ColorFillWebGpu(width, height, r, g, b, a),
        ucf_host_l2_raster_clear_webgpu: (width, height, r, g, b, a) => hostL2RasterClearWebGpu(width, height, r, g, b, a),
        ucf_host_l2_raster_tri_webgpu: (width, height) => hostL2RasterTriWebGpu(width, height),
    };
}
export function buildTonHostImports(memoryRef) {
    return {
        ton_host_present_rgba8_webgpu: (width, height, ptr, byteLen) => {
            const memory = memoryRef.current;
            if (!memory) {
                return -30;
            }
            return hostPresentRgba8WebGpu(width, height, memory, ptr, byteLen);
        },
    };
}
//# sourceMappingURL=index.js.map