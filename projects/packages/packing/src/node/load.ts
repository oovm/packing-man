import { createRequire } from "node:module";
import type { Problem, Solution, SolverId } from "../problem.js";

export const pmNodeRustTarget = "node-napi" as const;

export interface PmNapiBindings {
    pmVersionCode: () => number;
    solvePackingJson: (problemJson: string, solverJson: string) => string;
    renderSvgJson: (problemJson: string, solutionJson: string) => string;
}

const PLATFORM_PACKAGES: Record<string, string> = {
    "win32-x64": "@sxo/packing-win32-x64",
    "win32-arm64": "@sxo/packing-win32-arm64",
    "darwin-x64": "@sxo/packing-darwin-x64",
    "darwin-arm64": "@sxo/packing-darwin-arm64",
    "linux-x64": "@sxo/packing-linux-x64",
    "linux-arm64": "@sxo/packing-linux-arm64",
};

export async function loadPmNapi(): Promise<PmNapiBindings> {
    const key = `${process.platform}-${process.arch}`;
    const pkg = PLATFORM_PACKAGES[key];
    if (!pkg) throw new Error(`unsupported platform: ${key}`);
    const require = createRequire(import.meta.url);
    const native = require(pkg) as {
        pmVersionCode: () => number;
        solvePackingJson: (a: string, b: string) => string;
        renderSvgJson: (a: string, b: string) => string;
    };
    return {
        pmVersionCode: () => native.pmVersionCode(),
        solvePackingJson: (p, s) => native.solvePackingJson(p, s),
        renderSvgJson: (p, s) => native.renderSvgJson(p, s),
    };
}

export async function solvePackingNode(
    napi: PmNapiBindings,
    problem: Problem,
    solver: SolverId,
): Promise<Solution> {
    return JSON.parse(napi.solvePackingJson(JSON.stringify(problem), JSON.stringify(solver))) as Solution;
}

export async function renderSvgNode(
    napi: PmNapiBindings,
    problem: Problem,
    solution: Solution,
): Promise<string> {
    return napi.renderSvgJson(JSON.stringify(problem), JSON.stringify(solution));
}
