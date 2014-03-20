import {
    circlesInDisk,
    loadPmWasm,
    renderSvgWasm,
    solvePackingWasm,
    SolverPresets,
    pmWasmRustTarget,
    type LoadPmWasmOptions,
    type Problem,
    type Solution,
    type SolverId,
} from "@sxo/packing/wasm";

export const rustTarget = "wasm32-unknown-unknown" as const;
export const platformPackage = "packing-unknown-wasm32" as const;

const defaultWasmUrl = new URL("../lib/pm_wasm_bg.wasm", import.meta.url);

export async function loadPmWasmPlatform(options: LoadPmWasmOptions = {}) {
    return loadPmWasm({
        ...options,
        wasmUrl: options.wasmUrl ?? defaultWasmUrl,
    });
}

export async function solveAndRenderSvg(
    canvasHost: HTMLElement,
    options: {
        circleR?: number;
        containerR?: number;
        count?: number;
        solver?: SolverId;
        wasmUrl?: string | URL;
    } = {},
): Promise<{ solution: Solution; svg: string }> {
    const problem = circlesInDisk(
        options.circleR ?? 1,
        options.containerR ?? 10,
        options.count ?? 19,
    );
    const solver = options.solver ?? SolverPresets.cpuForce;
    const wasm = await loadPmWasmPlatform({ wasmUrl: options.wasmUrl });
    const solution = (await solvePackingWasm(wasm, problem, solver)) as Solution;
    const svg = await renderSvgWasm(wasm, problem, solution);
    canvasHost.innerHTML = svg;
    return { solution, svg };
}

export {
    pmWasmRustTarget,
    circlesInDisk,
    SolverPresets,
    loadPmWasm,
    solvePackingWasm,
    renderSvgWasm,
    type LoadPmWasmOptions,
    type Problem,
    type Solution,
    type SolverId,
};
