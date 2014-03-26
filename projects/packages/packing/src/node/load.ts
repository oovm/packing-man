import { createRequire } from "node:module";

import type { Checkpoint, SolveResumeResult } from "../checkpoint.js";

import type { Problem, Solution, SolverId } from "../problem.js";



export const pmNodeRustTarget = "node-napi" as const;



export interface PmNapiBindings {

    pmVersionCode: () => number;

    solvePackingJson: (problemJson: string, solverJson: string) => string;

    solvePackingResumeJson: (

        problemJson: string,

        solverJson: string,

        checkpointJson: string | null,

        iterBudget: number | null,

    ) => string;

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

        solvePackingResumeJson: (

            a: string,

            b: string,

            c: string | null,

            d: number | null,

        ) => string;

        renderSvgJson: (a: string, b: string) => string;

    };

    return {

        pmVersionCode: () => native.pmVersionCode(),

        solvePackingJson: (p, s) => native.solvePackingJson(p, s),

        solvePackingResumeJson: (p, s, c, b) => native.solvePackingResumeJson(p, s, c, b),

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



export interface SolveResumeOptions {

    checkpoint?: Checkpoint | null;

    iterBudget?: number;

}



export async function solvePackingResumeNode(

    napi: PmNapiBindings,

    problem: Problem,

    solver: SolverId,

    options: SolveResumeOptions = {},

): Promise<SolveResumeResult> {

    const checkpointJson =

        options.checkpoint != null ? JSON.stringify(options.checkpoint) : null;

    const iterBudget = options.iterBudget ?? null;

    return JSON.parse(

        napi.solvePackingResumeJson(

            JSON.stringify(problem),

            JSON.stringify(solver),

            checkpointJson,

            iterBudget,

        ),

    ) as SolveResumeResult;

}



export async function renderSvgNode(

    napi: PmNapiBindings,

    problem: Problem,

    solution: Solution,

): Promise<string> {

    return napi.renderSvgJson(JSON.stringify(problem), JSON.stringify(solution));

}


