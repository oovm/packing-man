import { initWebGpuHost } from "./ucf/index.js";
import { loadTonWasm } from "./load.js";
const DEFAULT_OUT_PTR = 65536;
export function presentRgba8ToCanvas2d(ctx, memory, outPtr, width, height) {
    const byteLen = width * height * 4;
    const bytes = new Uint8ClampedArray(byteLen);
    bytes.set(new Uint8Array(memory.buffer, outPtr, byteLen));
    ctx.putImageData(new ImageData(bytes, width, height), 0, 0);
}
function probeWebGpu() {
    return typeof navigator !== "undefined" && !!navigator.gpu;
}
async function presentL0Readback(canvas, memory, outPtr, width, height) {
    const ctx = canvas.getContext("2d");
    if (!ctx) {
        throw new Error("canvas 2d context unavailable");
    }
    presentRgba8ToCanvas2d(ctx, memory, outPtr, width, height);
}
async function presentL0Blit(target, memory, outPtr, width, height) {
    const scratch = document.createElement("canvas");
    scratch.width = width;
    scratch.height = height;
    await presentL0Readback(scratch, memory, outPtr, width, height);
    const ctx = target.getContext("2d");
    if (!ctx) {
        throw new Error("target canvas 2d unavailable for L0 blit");
    }
    ctx.drawImage(scratch, 0, 0);
}
/**
 * 渲染史瓦西黑洞一帧并上屏。
 * CPU 测地线在 `ton-simulation` / `ton-renderer`；上屏优先 UCF L2 WebGPU（`ton_l2_present_rgba8_webgpu` + `ucf_host`），降级 L0 readback。
 */
export async function renderBlackholeFrame(canvas, options = {}) {
    const width = canvas.width;
    const height = canvas.height;
    const mass = options.mass ?? 1.0;
    const outPtr = options.outPtr ?? DEFAULT_OUT_PTR;
    const byteLen = width * height * 4;
    const ton = await loadTonWasm(options);
    const status = ton.renderSchwarzschildRgba8(width, height, mass, outPtr);
    if (status !== 0) {
        throw new Error(`ton_render_schwarzschild_rgba8 failed: ${status}`);
    }
    if (outPtr + byteLen > ton.memory.buffer.byteLength) {
        throw new Error("ton wasm memory too small for frame readback");
    }
    if (!options.forceL0 && probeWebGpu()) {
        try {
            await initWebGpuHost(canvas);
            const l2 = ton.l2PresentRgba8Webgpu(width, height, outPtr, byteLen);
            if (l2 === 0) {
                return { ton, tier: "L2" };
            }
            throw new Error(`ton_l2_present_rgba8_webgpu failed: ${l2}`);
        }
        catch {
            // WebGPU canvas 可能已占用；降级 L0 blit
        }
    }
    try {
        await presentL0Readback(canvas, ton.memory, outPtr, width, height);
    }
    catch {
        await presentL0Blit(canvas, ton.memory, outPtr, width, height);
    }
    return { ton, tier: "L0" };
}
export { loadTonWasm } from "./load.js";
export { initWebGpuHost, UcfHostHandleRegistry } from "./ucf/index.js";
//# sourceMappingURL=present.js.map