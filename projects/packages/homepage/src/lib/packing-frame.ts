import {
    SolverPresets,
    circlesInDisk,
    isResumableSolver,
    loadPmWasm,
    renderSvgWasm,
    solvePackingResumeWasm,
    solvePackingWasm,
    type Checkpoint,
    type Problem,
    type Solution,
    type SolverId,
} from "@sxo/packing/wasm";

const WASM_URL = "/pm_wasm_bg.wasm";
const ITER_CHUNK = 50;

export interface PackCirclesOptions {
    containerR: number;
    circleR: number;
    count: number;
    algorithm: "greedy" | "force" | "gpu" | "concentric";
}

export interface PackResult {
    density: number;
    feasible: boolean;
    algorithm: string;
    objective: number;
    iterations: number;
}

export interface ProgressiveSolveCallbacks {
    onFrame?: (result: PackResult) => void;
    shouldContinue?: () => boolean;
    iterChunk?: number;
}

function solverFromFlag(algorithm: PackCirclesOptions["algorithm"]): SolverId {
    switch (algorithm) {
        case "greedy":
            return SolverPresets.cpuGreedy;
        case "concentric":
            return SolverPresets.cpuConcentric;
        case "gpu":
            return SolverPresets.gpuForce;
        case "force":
        default:
            return SolverPresets.cpuForce;
    }
}

export function solverForProblem(problem: Problem): SolverId {
    const archetype = problem.archetype;
    const objective = problem.objective;
    switch (archetype) {
        case "one_dim_bin_packing":
            return SolverPresets.cpuFfd;
        case "cubes_in_cubes":
        case "three_dim_bin_packing":
            return SolverPresets.cpuExtremePoint;
        case "two_dim_strip_packing":
            return SolverPresets.cpuSkyline;
        case "variable_circles_max_sum":
        case "variable_circles_in_perim_rect":
            return SolverPresets.cpuNlp;
        case "equal_circles_in_circle":
        case "congruent_circles_count_bound":
            return SolverPresets.cpuConcentric;
        case "equal_circles_in_square":
            if (objective === "max_equal_radius") {
                return SolverPresets.cpuNlp;
            }
            break;
    }
    if (objective === "max_radius_sum") {
        return SolverPresets.cpuNlp;
    }
    if (objective === "min_bins") {
        return SolverPresets.cpuFfd;
    }
    return SolverPresets.cpuForce;
}

function metricsFromSolution(solution: Solution): PackResult {
    const metrics = solution.metrics as Solution["metrics"] & {
        objective_value?: number;
        radius_sum?: number;
    };
    const objective = metrics.radius_sum ?? metrics.objective_value ?? metrics.density;
    return {
        density: metrics.density,
        feasible: solution.feasible,
        algorithm: solution.meta.algorithm,
        objective,
        iterations: solution.meta.iterations,
    };
}

function formatPackStatus(result: PackResult): string {
    return `${result.algorithm} · obj ${result.objective.toFixed(4)} · density ${(result.density * 100).toFixed(1)}% · ${result.iterations} iters · ${result.feasible ? "feasible" : "infeasible"}`;
}

function yieldToBrowser(): Promise<void> {
    return new Promise((resolve) => {
        requestAnimationFrame(() => setTimeout(resolve, 16));
    });
}

async function solveProgressive(
    host: HTMLElement,
    problem: Problem,
    solver: SolverId,
    callbacks: ProgressiveSolveCallbacks = {},
): Promise<PackResult> {
    const wasm = await loadPmWasm({ wasmUrl: WASM_URL });
    const iterChunk = callbacks.iterChunk ?? ITER_CHUNK;
    const shouldContinue = callbacks.shouldContinue ?? (() => true);

    if (!isResumableSolver(solver)) {
        const solution = (await solvePackingWasm(wasm, problem, solver)) as Solution;
        const svg = await renderSvgWasm(wasm, problem, solution);
        host.innerHTML = svg;
        const result = metricsFromSolution(solution);
        callbacks.onFrame?.(result);
        return result;
    }

    let checkpoint: Checkpoint | null = null;
    let lastResult: PackResult | null = null;
    let prevObjective: number | null = null;
    let stableFrames = 0;

    while (shouldContinue()) {
        const resume = await solvePackingResumeWasm(wasm, problem, solver, {
            checkpoint,
            iterBudget: iterChunk,
        });
        checkpoint = resume.checkpoint;
        const svg = await renderSvgWasm(wasm, problem, resume.solution);
        host.innerHTML = svg;
        lastResult = metricsFromSolution(resume.solution);
        callbacks.onFrame?.(lastResult);

        if (
            prevObjective !== null &&
            Math.abs(lastResult.objective - prevObjective) < 1e-10
        ) {
            stableFrames += 1;
            if (stableFrames >= 2) break;
        } else {
            stableFrames = 0;
            prevObjective = lastResult.objective;
        }

        await yieldToBrowser();
    }

    return lastResult!;
}

export async function runPackingDemo(
    host: HTMLElement,
    options: PackCirclesOptions,
    callbacks: ProgressiveSolveCallbacks = {},
): Promise<PackResult> {
    const problem = circlesInDisk(options.circleR, options.containerR, options.count);
    return solveProgressive(host, problem, solverFromFlag(options.algorithm), callbacks);
}

export async function runFixtureDemo(
    host: HTMLElement,
    fixtureUrl: string,
    callbacks: ProgressiveSolveCallbacks = {},
): Promise<PackResult & { fixtureId: string }> {
    const res = await fetch(fixtureUrl);
    if (!res.ok) {
        throw new Error(`fixture fetch failed: ${res.status}`);
    }
    const fixture = (await res.json()) as { id: string; problem: Problem };
    const result = await solveProgressive(host, fixture.problem, solverForProblem(fixture.problem), callbacks);
    return { ...result, fixtureId: fixture.id };
}

export { formatPackStatus };
