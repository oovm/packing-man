import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import type { Problem, Solution } from "./problem.js";

export interface FixtureExpect {
    objective_record?: number;
    bin_count?: number;
    optimality?: string;
    feasible?: boolean;
}

export interface Fixture {
    id: string;
    problem: Problem;
    expect?: FixtureExpect;
}

const FIXTURES_ROOT = join(
    dirname(fileURLToPath(import.meta.url)),
    "../../../crates/pm-benchmark/fixtures",
);

function readCatalogPaths(): string[] {
    const catalogPath = join(FIXTURES_ROOT, "catalog.json");
    const catalog = JSON.parse(readFileSync(catalogPath, "utf8")) as { fixtures: string[] };
    return catalog.fixtures.map((entry) => entry.replace(/\.json$/, ""));
}

export function allFixturePaths(): string[] {
    return readCatalogPaths();
}

export function loadFixtureByPath(path: string): Fixture | null {
    const normalized = path.replace(/\.json$/, "");
    const match = readCatalogPaths().find(
        (entry) => entry === normalized || entry.endsWith(`/${normalized}`) || entry.endsWith(normalized),
    );
    if (!match) {
        return null;
    }
    const full = join(FIXTURES_ROOT, `${match}.json`);
    return JSON.parse(readFileSync(full, "utf8")) as Fixture;
}

export function boundGap(fixture: Fixture, solution: Solution): number | null {
    const record = fixture.expect?.objective_record;
    if (record === undefined) {
        return null;
    }
    const achieved =
        (solution.metrics as { radius_sum?: number; objective_value?: number }).radius_sum ??
        (solution.metrics as { objective_value?: number }).objective_value ??
        0;
    return Math.max(0, record - achieved);
}
