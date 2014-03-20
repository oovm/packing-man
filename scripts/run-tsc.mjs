import { spawnSync } from "node:child_process";
import { createRequire } from "node:module";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const require = createRequire(resolve(repoRoot, "package.json"));
const tsc = require.resolve("typescript/lib/tsc.js");
const config = process.argv[2];
if (!config) {
    console.error("usage: node scripts/run-tsc.mjs <tsconfig.json>");
    process.exit(1);
}

const result = spawnSync(process.execPath, [tsc, "-p", resolve(process.cwd(), config)], {
    stdio: "inherit",
});
process.exit(result.status ?? 1);
