import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const distRuntime = process.argv[2];
if (!distRuntime) {
    console.error("usage: node patch-vmz-wasm-mime.mjs <path/to/dist/web-ssr/vmz-runtime.js>");
    process.exit(1);
}

const file = path.resolve(distRuntime);
if (!fs.existsSync(file)) {
    process.exit(0);
}

let src = fs.readFileSync(file, "utf8");
const needle = 'case ".map": return "application/json; charset=utf-8";';
const patch = `${needle}\n\t\tcase ".wasm": return "application/wasm";`;

if (src.includes('case ".wasm"')) {
    process.exit(0);
}
if (!src.includes(needle)) {
    console.warn(`patch-vmz-wasm-mime: pattern not found in ${file}`);
    process.exit(0);
}

src = src.replace(needle, patch);
fs.writeFileSync(file, src, "utf8");
console.log(`patched wasm MIME in ${path.relative(path.dirname(fileURLToPath(import.meta.url)), file)}`);
