import { buildTonHostImports, buildUcfHostImports, UcfHostHandleRegistry, } from "./ucf/index.js";
export const tonWasmRustTarget = "wasm32-unknown-unknown";
function decodeVersion(code) {
    const major = Math.floor(code / 1_000_000);
    const minor = Math.floor((code % 1_000_000) / 1_000);
    const patch = code % 1_000;
    return `${major}.${minor}.${patch}`;
}
function readCString(memory, ptr) {
    const view = new Uint8Array(memory.buffer);
    let end = ptr;
    while (view[end] !== 0) {
        end += 1;
    }
    return new TextDecoder().decode(view.subarray(ptr, end));
}
export async function loadTonWasm(options = {}) {
    const url = options.wasmUrl ?? new URL("../lib/ton_wasm_bg.wasm", import.meta.url);
    const memoryRef = {};
    const registry = options.registry ?? new UcfHostHandleRegistry();
    const imports = {
        ton_host: buildTonHostImports(memoryRef),
        ucf_host: buildUcfHostImports(registry),
    };
    let exports;
    if (options.module) {
        const instance = await WebAssembly.instantiate(options.module, imports);
        exports = instance.exports;
    }
    else if (typeof fetch !== "function") {
        throw new Error("fetch unavailable: provide wasm module or run in browser");
    }
    else {
        const result = await WebAssembly.instantiateStreaming(fetch(url.toString()), imports);
        exports = result.instance.exports;
    }
    if (!exports.memory || !exports.ton_render_schwarzschild_rgba8) {
        throw new Error("ton_wasm_bg.wasm unavailable: run build:wasm first");
    }
    memoryRef.current = exports.memory;
    const memory = exports.memory;
    return {
        memory,
        info: () => {
            const diagPtr = 512;
            let ucfDiagMask = 0;
            const diagStatus = exports.ton_ucf_application_contract_diag_mask(diagPtr);
            if (diagStatus === 0) {
                ucfDiagMask = new DataView(memory.buffer).getUint32(diagPtr, true);
            }
            return {
                name: "TON Blackhole Simulator",
                version: decodeVersion(exports.ton_version_code()),
                versionCode: exports.ton_version_code(),
                targetTriple: readCString(memory, exports.ton_target_triple()),
                ucfDiagMask,
            };
        },
        renderSchwarzschildRgba8: (width, height, mass, outPtr) => exports.ton_render_schwarzschild_rgba8(width, height, mass, outPtr),
        l2PresentRgba8Webgpu: (width, height, ptr, byteLen) => {
            if (!exports.ton_l2_present_rgba8_webgpu) {
                return -50;
            }
            return exports.ton_l2_present_rgba8_webgpu(width, height, ptr, byteLen);
        },
        ucfL2RasterTriWebgpu: (width, height) => {
            if (!exports.ton_ucf_l2_raster_tri_webgpu) {
                return -50;
            }
            return exports.ton_ucf_l2_raster_tri_webgpu(width, height);
        },
    };
}
//# sourceMappingURL=load.js.map