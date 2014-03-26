import { readFileSync, writeFileSync } from "node:fs";



import type { Checkpoint } from "./checkpoint.js";

import { allFixturePaths, boundGap, loadFixtureByPath, type Fixture } from "./fixtures.js";

import { circlesInDisk, type Problem, type Solution, type SolverId } from "./problem.js";

import { loadPmNapi, renderSvgNode, solvePackingResumeNode } from "./node/load.js";

import { solverForProblem, solverFromAlgorithmFlag } from "./solver.js";



function flagValue(args: string[], flag: string): string | undefined {

    const index = args.indexOf(flag);

    if (index < 0) {

        return undefined;

    }

    return args[index + 1];

}



function parseNumber(args: string[], flag: string, defaultValue?: number): number | undefined {

    const raw = flagValue(args, flag);

    if (raw === undefined) {

        return defaultValue;

    }

    const value = Number(raw);

    return Number.isFinite(value) ? value : defaultValue;

}



function parseCount(args: string[], flag: string, defaultValue: number): number {

    const value = Math.trunc(parseNumber(args, flag, defaultValue) ?? defaultValue);

    return value > 0 ? value : defaultValue;

}



function loadCheckpoint(path: string): Checkpoint {

    return JSON.parse(readFileSync(path, "utf8")) as Checkpoint;

}



interface SolveRun {

    code: number;

    solution?: Solution;

    checkpoint?: Checkpoint;

}



async function runSolve(

    problem: Problem,

    solver: SolverId,

    out: string,

    options: {

        fixture?: Fixture;

        checkpointIn?: string;

        checkpointOut?: string;

        iterBudget?: number;

    } = {},

): Promise<SolveRun> {

    const napi = await loadPmNapi();

    let solution: Solution;

    let checkpoint: Checkpoint | undefined;

    try {

        const resume = await solvePackingResumeNode(napi, problem, solver, {

            checkpoint: options.checkpointIn ? loadCheckpoint(options.checkpointIn) : null,

            iterBudget: options.iterBudget,

        });

        solution = resume.solution;

        checkpoint = resume.checkpoint;

    } catch (error) {

        console.error(`solve failed: ${error instanceof Error ? error.message : String(error)}`);

        return { code: 2 };

    }



    if (options.checkpointOut) {

        writeFileSync(options.checkpointOut, JSON.stringify(checkpoint, null, 2), "utf8");

    }



    try {

        const svg = await renderSvgNode(napi, problem, solution);

        writeFileSync(out, svg, "utf8");

    } catch (error) {

        console.error(`write failed: ${error instanceof Error ? error.message : String(error)}`);

        return { code: 1 };

    }



    const metrics = solution.metrics as Solution["metrics"] & {

        radius_sum?: number;

        objective_value?: number;

    };

    const gap = options.fixture ? boundGap(options.fixture, solution) : null;

    const gapMsg = gap != null ? ` bound_gap=${gap.toFixed(6)}` : "";

    const prefix = options.fixture ? `id=${options.fixture.id} ` : "";

    const cpMsg = checkpoint

        ? ` iterations=${checkpoint.total_iterations} checkpoint_budget=${checkpoint.iter_budget}`

        : "";

    console.error(

        `wrote ${out} — ${prefix}count=${metrics.count} density=${(metrics.density * 100).toFixed(2)}% radius_sum=${(metrics.radius_sum ?? 0).toFixed(6)} feasible=${solution.feasible} obj=${(metrics.objective_value ?? 0).toFixed(6)}${gapMsg}${cpMsg}`,

    );

    return { code: 0, solution, checkpoint };

}



export async function runCli(argv: string[]): Promise<number> {

    const args = argv.slice(2);



    if (args.includes("--list-fixtures")) {

        for (const path of allFixturePaths()) {

            console.log(path);

        }

        return 0;

    }



    const checkpointIn = flagValue(args, "--checkpoint-in");

    const checkpointOut = flagValue(args, "--checkpoint-out");

    const iterBudget = parseNumber(args, "--iter-budget");



    const fixturePath = flagValue(args, "--fixture");

    if (fixturePath) {

        const fixture = loadFixtureByPath(fixturePath);

        if (!fixture) {

            console.error(`unknown fixture: ${fixturePath}`);

            console.error("use --list-fixtures to see paths");

            return 1;

        }

        const out = flagValue(args, "--out") ?? "packing.svg";

        const solver = solverForProblem(fixture.problem);

        const result = await runSolve(fixture.problem, solver, out, {

            fixture,

            checkpointIn,

            checkpointOut,

            iterBudget,

        });

        return result.code;

    }



    const out = args[0] ?? "packing.svg";

    const containerR = parseNumber(args, "--container-r", 10) ?? 10;

    const circleR = parseNumber(args, "--circle-r", 1) ?? 1;

    const count = parseCount(args, "--count", 19);

    const algorithm = flagValue(args, "--algorithm") ?? "force";

    const problem = circlesInDisk(circleR, containerR, count);

    const solver = solverFromAlgorithmFlag(algorithm);

    const result = await runSolve(problem, solver, out, {

        checkpointIn,

        checkpointOut,

        iterBudget,

    });

    return result.code;

}


