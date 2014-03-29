import {
    SolverPresets,
    circlesInDisk,
    loadPmWasm,
    renderSvgWasm,
    solvePackingWasm,
    type Problem,
    type Solution,
    type SolverId,
} from "@sxo/packing/wasm";

const WASM_URL = "/pm_wasm_bg.wasm";

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

async function solveAndPaint(
    host: HTMLElement,
    problem: Problem,
    solver: SolverId,
): Promise<PackResult> {
    const wasm = await loadPmWasm({ wasmUrl: WASM_URL });
    const solution = (await solvePackingWasm(wasm, problem, solver)) as Solution;
    const svg = await renderSvgWasm(wasm, problem, solution);
    host.innerHTML = svg;
    return metricsFromSolution(solution);
}

export async function runPackingDemo(
    host: HTMLElement,
    options: PackCirclesOptions,
): Promise<PackResult> {
    const problem = circlesInDisk(options.circleR, options.containerR, options.count);
    return solveAndPaint(host, problem, solverFromFlag(options.algorithm));
}

export async function runFixtureDemo(
    host: HTMLElement,
    fixtureUrl: string,
): Promise<PackResult & { fixtureId: string }> {
    const res = await fetch(fixtureUrl);
    if (!res.ok) {
        throw new Error(`fixture fetch failed: ${res.status}`);
    }
    const fixture = (await res.json()) as { id: string; problem: Problem };
    const result = await solveAndPaint(host, fixture.problem, solverForProblem(fixture.problem));
    return { ...result, fixtureId: fixture.id };
}
