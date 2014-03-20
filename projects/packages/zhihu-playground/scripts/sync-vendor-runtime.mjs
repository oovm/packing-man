#!/usr/bin/env node
/**
 * Copy built wasm loader JS into vendor/runtime (not `dist/`, excluded by CloudBase ZIP rules)
 * and refresh wasm bytes for npm + static hosting.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(here, "..");
const repo = path.resolve(root, "../../..");
const wasmBag = path.join(repo, "projects/packages/packing-unknown-wasm32");
const vendor = path.join(root, "vendor/packing-unknown-wasm32");
const srcDist = path.join(wasmBag, "dist");
const dstRuntime = path.join(vendor, "runtime");
const wasmSrc = path.join(wasmBag, "lib/pm_wasm_bg.wasm");

function copyTree(src, dst) {
    fs.mkdirSync(dst, { recursive: true });
    for (const name of fs.readdirSync(src, { withFileTypes: true })) {
        const from = path.join(src, name.name);
        const to = path.join(dst, name.name);
        if (name.isDirectory()) {
            copyTree(from, to);
        } else {
            fs.copyFileSync(from, to);
        }
    }
}

if (!fs.existsSync(srcDist)) {
    console.error(`missing ${srcDist} — run: pnpm --filter @sxo/packing-unknown-wasm32 run build`);
    process.exit(1);
}
if (!fs.existsSync(wasmSrc)) {
    console.error(`missing ${wasmSrc} — run: pnpm --filter @sxo/packing-unknown-wasm32 run build:wasm`);
    process.exit(1);
}

fs.rmSync(dstRuntime, { recursive: true, force: true });
copyTree(srcDist, dstRuntime);

const vendorWasm = path.join(vendor, "lib/pm_wasm_bg.wasm");
const publicWasm = path.join(root, "public/pm_wasm_bg.wasm");
fs.mkdirSync(path.dirname(vendorWasm), { recursive: true });
fs.mkdirSync(path.dirname(publicWasm), { recursive: true });
fs.copyFileSync(wasmSrc, vendorWasm);
fs.copyFileSync(wasmSrc, publicWasm);
console.log(`synced vendor runtime from ${srcDist}`);
console.log(`wasm → ${vendorWasm} and ${publicWasm}`);
