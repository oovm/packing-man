import { execSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const wasmTarget = "wasm32-unknown-unknown";
const cargoWasm = join(root, "target", wasmTarget, "release", "pm_wasm.wasm");
const bagWasm = join(root, "projects/packages/packing-unknown-wasm32/lib/pm_wasm_bg.wasm");
const homepageWasm = join(root, "projects/packages/homepage/public/pm_wasm_bg.wasm");

const copyOnly = process.argv.includes("--copy-only");
const copyHomepage = process.argv.includes("--copy-homepage");

function copyWasm() {
    if (!existsSync(cargoWasm)) {
        console.error(`missing ${cargoWasm}`);
        console.error(`run: node scripts/build-wasm.mjs`);
        process.exit(1);
    }
    mkdirSync(dirname(bagWasm), { recursive: true });
    copyFileSync(cargoWasm, bagWasm);
    console.log(`copied → ${bagWasm}`);
    if (copyHomepage) {
        mkdirSync(dirname(homepageWasm), { recursive: true });
        copyFileSync(cargoWasm, homepageWasm);
        console.log(`copied → ${homepageWasm}`);
    }
}

if (!copyOnly) {
    execSync(`cargo build -p pm-wasm --target ${wasmTarget} --release`, {
        cwd: root,
        stdio: "inherit",
        shell: true,
    });
}

copyWasm();
