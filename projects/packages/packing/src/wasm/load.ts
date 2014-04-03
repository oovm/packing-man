export const pmWasmRustTarget = "wasm32-unknown-unknown" as const;

import type { Checkpoint, SolveResumeResult } from "../checkpoint.js";

export interface PmWasmBindings {
    pm_version_code: () => number;
    pm_solve_json: (inPtr: number, inLen: number, outPtr: number, outCap: number) => number;
    pm_solve_resume_json: (inPtr: number, inLen: number, outPtr: number, outCap: number) => number;
    pm_render_svg: (inPtr: number, inLen: number, outPtr: number, outCap: number) => number;
    memory: WebAssembly.Memory;
}

export interface LoadPmWasmOptions {
    wasmUrl?: string | URL;
    module?: WebAssembly.Module;
}

export async function loadPmWasm(options: LoadPmWasmOptions = {}): Promise<PmWasmBindings> {
    if (!options.wasmUrl && !options.module) {
        throw new Error("loadPmWasm requires wasmUrl or module");
    }
    let instance: WebAssembly.Instance;
    if (options.module) {
        const instantiated: unknown = await WebAssembly.instantiate(options.module, {});
        instance =
            instantiated instanceof WebAssembly.Instance
                ? instantiated
                : (instantiated as WebAssembly.WebAssemblyInstantiatedSource).instance;
    } else {
        const url = options.wasmUrl!.toString();
        try {
            const result = await WebAssembly.instantiateStreaming(fetch(url), {});
            instance = result.instance;
        } catch {
            const response = await fetch(url);
            if (!response.ok) {
                throw new Error(`wasm fetch failed: ${response.status}`);
            }
            const bytes = await response.arrayBuffer();
            const result = await WebAssembly.instantiate(bytes, {});
            instance = result.instance;
        }
    }
    const exports = instance.exports as unknown as PmWasmBindings;
    if (!exports.memory || !exports.pm_solve_json || !exports.pm_solve_resume_json) {
        throw new Error("pm_wasm missing exports — run pnpm build:wasm");
    }
    return exports;
}

function writeJson(memory: WebAssembly.Memory, json: string, outCap = 65536): number {
    const bytes = new TextEncoder().encode(json);
    const ptr = 65536;
    if (bytes.length > outCap) throw new Error("json too large for wasm buffer");
    new Uint8Array(memory.buffer, ptr, bytes.length).set(bytes);
    return ptr;
}

function readUtf8(memory: WebAssembly.Memory, ptr: number, len: number): string {
    return new TextDecoder().decode(new Uint8Array(memory.buffer, ptr, len));
}

export async function solvePackingWasm(
    wasm: PmWasmBindings,
    problem: unknown,
    solver: unknown,
): Promise<unknown> {
    const req = JSON.stringify({ problem, solver });
    const inPtr = writeJson(wasm.memory, req);
    const outPtr = 131072;
    const outCap = 262144;
    const written = wasm.pm_solve_json(inPtr, req.length, outPtr, outCap);
    if (written < 0) throw new Error(`pm_solve_json failed: ${written}`);
    return JSON.parse(readUtf8(wasm.memory, outPtr, written));
}

export interface SolveResumeWasmOptions {
    checkpoint?: Checkpoint | null;
    iterBudget?: number;
}

export async function solvePackingResumeWasm(
    wasm: PmWasmBindings,
    problem: unknown,
    solver: unknown,
    options: SolveResumeWasmOptions = {},
): Promise<SolveResumeResult> {
    const req = JSON.stringify({
        problem,
        solver,
        checkpoint: options.checkpoint ?? null,
        iter_budget: options.iterBudget ?? null,
    });
    const inPtr = writeJson(wasm.memory, req);
    const outPtr = 131072;
    const outCap = 524288;
    const written = wasm.pm_solve_resume_json(inPtr, req.length, outPtr, outCap);
    if (written < 0) throw new Error(`pm_solve_resume_json failed: ${written}`);
    return JSON.parse(readUtf8(wasm.memory, outPtr, written)) as SolveResumeResult;
}

export async function renderSvgWasm(
    wasm: PmWasmBindings,
    problem: unknown,
    solution: unknown,
): Promise<string> {
    const req = JSON.stringify({ problem, solution });
    const inPtr = writeJson(wasm.memory, req);
    const outPtr = 131072;
    const outCap = 524288;
    const written = wasm.pm_render_svg(inPtr, req.length, outPtr, outCap);
    if (written < 0) throw new Error(`pm_render_svg failed: ${written}`);
    return readUtf8(wasm.memory, outPtr, written);
}
