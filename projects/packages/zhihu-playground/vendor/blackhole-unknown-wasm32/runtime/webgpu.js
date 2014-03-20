/**
 * WebGPU 宿主：上传 wasm RGBA8 并绘制全屏四边形。
 */
let hostState;
const TEXTURE_WGSL = `
struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VSOut {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
    );
    var uv = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0),
    );
    var o: VSOut;
    o.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    o.uv = uv[vid];
    return o;
}

@group(0) @binding(0) var texSampler: sampler;
@group(0) @binding(1) var texColor: texture_2d<f32>;

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(texColor, texSampler, uv);
}
`;
export async function initTonWebGpuHost(canvas) {
    const gpu = navigator.gpu;
    if (!gpu) {
        throw new Error("WebGPU unavailable");
    }
    const adapter = await gpu.requestAdapter();
    if (!adapter) {
        throw new Error("WebGPU adapter unavailable");
    }
    const device = await adapter.requestDevice();
    const context = canvas.getContext("webgpu");
    if (!context) {
        throw new Error("webgpu canvas context unavailable");
    }
    const format = gpu.getPreferredCanvasFormat();
    context.configure({ device, format, alphaMode: "premultiplied" });
    const module = device.createShaderModule({ code: TEXTURE_WGSL });
    const pipeline = device.createRenderPipeline({
        layout: "auto",
        vertex: { module, entryPoint: "vs_main" },
        fragment: {
            module,
            entryPoint: "fs_main",
            targets: [{ format }],
        },
        primitive: { topology: "triangle-list" },
    });
    const sampler = device.createSampler({ magFilter: "linear", minFilter: "linear" });
    hostState = { device, context, format, pipeline, sampler };
    return hostState;
}
export function getTonWebGpuState() {
    return hostState;
}
/** wasm import `ton_host_present_rgba8_webgpu` 实现。 */
export function hostPresentRgba8WebGpu(width, height, memory, ptr, byteLen) {
    if (width === 0 || height === 0) {
        return 0;
    }
    const state = hostState;
    if (!state) {
        return -30;
    }
    try {
        const bytes = new Uint8Array(memory.buffer, ptr, byteLen);
        const texture = state.device.createTexture({
            size: [width, height],
            format: "rgba8unorm",
            usage: GPUTextureUsage.TEXTURE_BINDING |
                GPUTextureUsage.COPY_DST |
                GPUTextureUsage.RENDER_ATTACHMENT,
        });
        state.device.queue.writeTexture({ texture }, bytes, { bytesPerRow: width * 4 }, [width, height]);
        const bindGroup = state.device.createBindGroup({
            layout: state.pipeline.getBindGroupLayout(0),
            entries: [
                { binding: 0, resource: state.sampler },
                { binding: 1, resource: texture.createView() },
            ],
        });
        const swap = state.context.getCurrentTexture();
        const encoder = state.device.createCommandEncoder();
        const pass = encoder.beginRenderPass({
            colorAttachments: [
                {
                    view: swap.createView(),
                    clearValue: { r: 0, g: 0, b: 0, a: 1 },
                    loadOp: "clear",
                    storeOp: "store",
                },
            ],
        });
        pass.setPipeline(state.pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.draw(6);
        pass.end();
        state.device.queue.submit([encoder.finish()]);
        return 0;
    }
    catch {
        return -31;
    }
}
//# sourceMappingURL=webgpu.js.map