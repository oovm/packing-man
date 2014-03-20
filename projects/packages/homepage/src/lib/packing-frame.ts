import {
    SolverPresets,
    type SolverId,
    solveAndRenderSvg,
} from "@sxo/packing-unknown-wasm32";

const WASM_URL = "/pm_wasm_bg.wasm";

export interface PackCirclesOptions {
    containerR: number;
    circleR: number;
    count: number;
    algorithm: "greedy" | "force" | "gpu";
}

export async function runPackingDemo(
    host: HTMLElement,
    options: PackCirclesOptions,
): Promise<{ density: number; feasible: boolean; algorithm: string }> {
    const solver: SolverId =
        options.algorithm === "greedy"
            ? SolverPresets.cpuGreedy
            : options.algorithm === "gpu"
              ? SolverPresets.gpuForce
              : SolverPresets.cpuForce;
    const { solution } = await solveAndRenderSvg(host, {
        circleR: options.circleR,
        containerR: options.containerR,
        count: options.count,
        solver,
        wasmUrl: WASM_URL,
    });
    return {
        density: solution.metrics.density,
        feasible: solution.feasible,
        algorithm: solution.meta.algorithm,
    };
}
