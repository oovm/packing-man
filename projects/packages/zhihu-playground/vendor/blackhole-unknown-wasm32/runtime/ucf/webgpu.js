/**
 * UCF L2 宿主 WebGPU 桥（浏览器 API，非 Rust `wgpu` crate）。
 * WGSL 仅用于 L2 宿主 companion（与 `ucf_emitter` 薄门对齐），不是产品测地线着色器。
 */
let hostState;
/** 与 `ucf_emitter` / CPU `raster_tri` 薄门对齐的 WGSL（`vertex_index` 大三角）。 */
const RASTER_TRI_WGSL = `
struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) col: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VSOut {
    var verts = array<vec2<f32>, 3>(
        vec2<f32>(-0.8, -0.8),
        vec2<f32>( 0.8, -0.8),
        vec2<f32>( 0.0,  0.8)
    );
    var o: VSOut;
    o.pos = vec4<f32>(verts[vid], 0.0, 1.0);
    o.col = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    return o;
}

@fragment
fn fs_main(@location(0) col: vec4<f32>) -> @location(0) vec4<f32> {
    return col;
}
`;
/** RGBA8 纹理 blit 到 swapchain（`ton_host_present_rgba8_webgpu` 宿主 companion）。 */
const RGBA8_BLIT_WGSL = `
struct VSOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@vertex
fn vs_blit(@builtin(vertex_index) vid: u32) -> VSOut {
    var pos = array<vec2<f32>, 6>(
        vec2<f32>(-1.0,  1.0), vec2<f32>(-1.0, -1.0), vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0, -1.0), vec2<f32>( 1.0, -1.0), vec2<f32>( 1.0,  1.0)
    );
    var uv = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0), vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 0.0),
        vec2<f32>(0.0, 1.0), vec2<f32>(1.0, 1.0), vec2<f32>(1.0, 0.0)
    );
    var o: VSOut;
    o.pos = vec4<f32>(pos[vid], 0.0, 1.0);
    o.uv = uv[vid];
    return o;
}

@group(0) @binding(0) var frameTex: texture_2d<f32>;
@group(0) @binding(1) var frameSampler: sampler;

@fragment
fn fs_blit(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(frameTex, frameSampler, uv);
}
`;
let rasterTriPipeline;
let rgba8BlitPipeline;
let rgba8BlitBindGroupLayout;
let rgba8BlitSampler;
export async function initWebGpuHost(canvas) {
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
    context.configure({
        device,
        format,
        alphaMode: "premultiplied",
    });
    hostState = { device, context, format };
    return hostState;
}
export function getWebGpuHostState() {
    return hostState;
}
export function hostL2ColorFillWebGpu(width, height, r, g, b, a) {
    if (width === 0 || height === 0) {
        return 0;
    }
    const state = hostState;
    if (!state) {
        return -30;
    }
    try {
        const texture = state.context.getCurrentTexture();
        const encoder = state.device.createCommandEncoder();
        const pass = encoder.beginRenderPass({
            colorAttachments: [
                {
                    view: texture.createView(),
                    clearValue: {
                        r: r / 255,
                        g: g / 255,
                        b: b / 255,
                        a: a / 255,
                    },
                    loadOp: "clear",
                    storeOp: "store",
                },
            ],
        });
        pass.end();
        state.device.queue.submit([encoder.finish()]);
        return 0;
    }
    catch {
        return -31;
    }
}
export function hostL2RasterClearWebGpu(width, height, r, g, b, a) {
    return hostL2ColorFillWebGpu(width, height, r, g, b, a);
}
function ensureRasterTriPipeline(state) {
    if (rasterTriPipeline) {
        return rasterTriPipeline;
    }
    const module = state.device.createShaderModule({ code: RASTER_TRI_WGSL });
    rasterTriPipeline = state.device.createRenderPipeline({
        layout: "auto",
        vertex: { module, entryPoint: "vs_main" },
        fragment: { module, entryPoint: "fs_main", targets: [{ format: state.format }] },
        primitive: { topology: "triangle-list" },
    });
    return rasterTriPipeline;
}
export function hostL2RasterTriWebGpu(width, height) {
    if (width === 0 || height === 0) {
        return 0;
    }
    const state = hostState;
    if (!state) {
        return -30;
    }
    try {
        const texture = state.context.getCurrentTexture();
        const pipeline = ensureRasterTriPipeline(state);
        const encoder = state.device.createCommandEncoder();
        const pass = encoder.beginRenderPass({
            colorAttachments: [
                {
                    view: texture.createView(),
                    clearValue: { r: 0, g: 0, b: 0, a: 1 },
                    loadOp: "clear",
                    storeOp: "store",
                },
            ],
        });
        pass.setPipeline(pipeline);
        pass.draw(3);
        pass.end();
        state.device.queue.submit([encoder.finish()]);
        return 0;
    }
    catch {
        return -31;
    }
}
function ensureRgba8BlitResources(state) {
    if (rgba8BlitPipeline && rgba8BlitBindGroupLayout && rgba8BlitSampler) {
        return {
            pipeline: rgba8BlitPipeline,
            layout: rgba8BlitBindGroupLayout,
            sampler: rgba8BlitSampler,
        };
    }
    rgba8BlitSampler = state.device.createSampler({
        magFilter: "nearest",
        minFilter: "nearest",
    });
    rgba8BlitBindGroupLayout = state.device.createBindGroupLayout({
        entries: [
            { binding: 0, visibility: GPUShaderStage.FRAGMENT, texture: {} },
            { binding: 1, visibility: GPUShaderStage.FRAGMENT, sampler: {} },
        ],
    });
    const module = state.device.createShaderModule({ code: RGBA8_BLIT_WGSL });
    rgba8BlitPipeline = state.device.createRenderPipeline({
        layout: state.device.createPipelineLayout({
            bindGroupLayouts: [rgba8BlitBindGroupLayout],
        }),
        vertex: { module, entryPoint: "vs_blit" },
        fragment: { module, entryPoint: "fs_blit", targets: [{ format: state.format }] },
        primitive: { topology: "triangle-list" },
    });
    return {
        pipeline: rgba8BlitPipeline,
        layout: rgba8BlitBindGroupLayout,
        sampler: rgba8BlitSampler,
    };
}
/** wasm import `ton_host_present_rgba8_webgpu`：wasm 线性内存 RGBA8 → WebGPU swapchain。 */
export function hostPresentRgba8WebGpu(width, height, memory, ptr, byteLen) {
    if (width === 0 || height === 0) {
        return 0;
    }
    const state = hostState;
    if (!state) {
        return -30;
    }
    const need = width * height * 4;
    if (byteLen < need || ptr + need > memory.buffer.byteLength) {
        return -2;
    }
    try {
        const bytes = new Uint8Array(memory.buffer, ptr, need);
        const gpuBuffer = state.device.createBuffer({
            size: need,
            usage: GPUBufferUsage.COPY_SRC | GPUBufferUsage.COPY_DST,
        });
        state.device.queue.writeBuffer(gpuBuffer, 0, bytes);
        const frameTexture = state.device.createTexture({
            size: [width, height],
            format: "rgba8unorm",
            usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.TEXTURE_BINDING,
        });
        const encoder = state.device.createCommandEncoder();
        encoder.copyBufferToTexture({ buffer: gpuBuffer, bytesPerRow: width * 4, rowsPerImage: height }, { texture: frameTexture }, [width, height]);
        const { pipeline, layout, sampler } = ensureRgba8BlitResources(state);
        const bindGroup = state.device.createBindGroup({
            layout,
            entries: [
                { binding: 0, resource: frameTexture.createView() },
                { binding: 1, resource: sampler },
            ],
        });
        const swap = state.context.getCurrentTexture();
        const pass = encoder.beginRenderPass({
            colorAttachments: [
                {
                    view: swap.createView(),
                    loadOp: "clear",
                    clearValue: { r: 0, g: 0, b: 0, a: 1 },
                    storeOp: "store",
                },
            ],
        });
        pass.setPipeline(pipeline);
        pass.setBindGroup(0, bindGroup);
        pass.draw(6);
        pass.end();
        state.device.queue.submit([encoder.finish()]);
        gpuBuffer.destroy();
        frameTexture.destroy();
        return 0;
    }
    catch {
        return -31;
    }
}
//# sourceMappingURL=webgpu.js.map