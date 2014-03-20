import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, "..");
const src = path.join(here, "vmz", "host-runtime-files.json");
const pnpmDir = path.join(repoRoot, "node_modules", ".pnpm");

if (!fs.existsSync(src)) {
    console.error(`missing vendored manifest: ${src}`);
    process.exit(1);
}

/** @returns {string[]} */
function findVmzPackageRoots() {
    if (!fs.existsSync(pnpmDir)) {
        return [];
    }
    const roots = [];
    for (const ent of fs.readdirSync(pnpmDir)) {
        if (!ent.startsWith("@vmz+vmz@")) {
            continue;
        }
        const pkg = path.join(pnpmDir, ent, "node_modules", "@vmz", "vmz");
        if (fs.existsSync(path.join(pkg, "package.json"))) {
            roots.push(pkg);
        }
    }
    return roots;
}

let patched = 0;
for (const vmzRoot of findVmzPackageRoots()) {
    const dst = path.join(vmzRoot, "host-runtime-files.json");
    if (!fs.existsSync(dst)) {
        fs.copyFileSync(src, dst);
        patched += 1;
        console.log(`patched @vmz/vmz host-runtime-files.json → ${dst}`);
    }
}

if (patched === 0 && findVmzPackageRoots().length === 0) {
    // @vmz/vmz not installed yet (homepage-only dep).
    process.exit(0);
}
