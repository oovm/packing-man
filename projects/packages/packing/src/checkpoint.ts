import type { Problem, Solution, SolverId } from "./problem.js";

export interface Checkpoint {
    version: number;
    problem: Problem;
    solver: SolverId;
    total_iterations: number;
    iter_budget: number;
    solution: Solution;
}

export interface SolveResumeResult {
    solution: Solution;
    checkpoint: Checkpoint;
}
