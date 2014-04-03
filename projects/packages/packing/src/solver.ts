import { AlgorithmKind, Backend, SolverPresets, type SolverId } from "./problem.js";

export function isResumableSolver(solver: SolverId): boolean {
    const { backend, algorithm } = solver;
    if (backend === Backend.cpu && algorithm === AlgorithmKind.force_relaxation) return true;
    if (backend === Backend.cpu && algorithm === AlgorithmKind.nlp_local_search) return true;
    if (backend === Backend.gpu && algorithm === AlgorithmKind.gpu_force_relaxation) return true;
    return false;
}

export function solverFromAlgorithmFlag(flag: string): SolverId {
    switch (flag) {
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

interface ProblemRoute {
    archetype?: string;
    objective?: string;
}

export function solverForProblem(problem: ProblemRoute): SolverId {
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
