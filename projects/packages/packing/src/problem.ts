/** 与 `pm-types` JSON 对齐。 */

export const ProblemFamily = {
    manufacturer_pallet_loading: "manufacturer_pallet_loading",
    distributor_pallet_loading: "distributor_pallet_loading",
    circle_sphere_packing: "circle_sphere_packing",
    convex_region_packing: "convex_region_packing",
} as const;

export type ProblemFamily = (typeof ProblemFamily)[keyof typeof ProblemFamily];

export const Objective = {
    max_count: "max_count",
    min_container_size: "min_container_size",
    min_bins: "min_bins",
    max_value: "max_value",
    feasibility: "feasibility",
} as const;

export type Objective = (typeof Objective)[keyof typeof Objective];

export const AlgorithmKind = {
    greedy_insertion: "greedy_insertion",
    force_relaxation: "force_relaxation",
    nlp_sqp: "nlp_sqp",
    gpu_force_relaxation: "gpu_force_relaxation",
} as const;

export type AlgorithmKind = (typeof AlgorithmKind)[keyof typeof AlgorithmKind];

export const Backend = { cpu: "cpu", gpu: "gpu" } as const;
export type Backend = (typeof Backend)[keyof typeof Backend];

export interface SolverId {
    family: ProblemFamily;
    backend: Backend;
    algorithm: AlgorithmKind;
}

export const SolverPresets = {
    cpuGreedy: {
        family: ProblemFamily.circle_sphere_packing,
        backend: Backend.cpu,
        algorithm: AlgorithmKind.greedy_insertion,
    },
    cpuForce: {
        family: ProblemFamily.circle_sphere_packing,
        backend: Backend.cpu,
        algorithm: AlgorithmKind.force_relaxation,
    },
    gpuForce: {
        family: ProblemFamily.circle_sphere_packing,
        backend: Backend.gpu,
        algorithm: AlgorithmKind.gpu_force_relaxation,
    },
} as const satisfies Record<string, SolverId>;

export interface ItemSpec {
    model: { kind: "circle"; radius: number; count: number };
}

export interface Problem {
    family: ProblemFamily;
    objective: Objective;
    items: ItemSpec;
    container: { kind: "circle"; radius: number } | { kind: "rectangle"; width: number; height: number };
    rules: { rules: string[] };
}

export interface Placement2d {
    id: number;
    cx: number;
    cy: number;
    radius: number;
}

export interface Solution {
    family: ProblemFamily;
    placements: Placement2d[];
    metrics: { count: number; density: number; container_area: number; packed_area: number };
    feasible: boolean;
    meta: { algorithm: string; backend: string; iterations: number; elapsed_ms: number };
}

export function circlesInDisk(circleR: number, containerR: number, count: number): Problem {
    return {
        family: ProblemFamily.circle_sphere_packing,
        objective: Objective.max_count,
        items: { model: { kind: "circle", radius: circleR, count } },
        container: { kind: "circle", radius: containerR },
        rules: { rules: [] },
    };
}
