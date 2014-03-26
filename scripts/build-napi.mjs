import { execSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, readdirSync, unlinkSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const packagesDir = join(root, "projects/packages");

/** @type {Array<{ hostKey: string, target?: string, cargoRel: string, bag: string, fileName: string }>} */
const ARTIFACTS = [
    {
        hostKey: "win32-x64",
        cargoRel: "target/release/pm_napi.dll",
        bag: "packing-win32-x64",
        fileName: "pm.win32-x64-msvc.node",
    },
    {
        hostKey: "win32-arm64",
        target: "aarch64-pc-windows-msvc",
        cargoRel: "target/aarch64-pc-windows-msvc/release/pm_napi.dll",
        bag: "packing-win32-arm64",
        fileName: "pm.win32-arm64-msvc.node",
    },
    {
        hostKey: "linux-x64",
        target: "x86_64-unknown-linux-gnu",
        cargoRel: "target/x86_64-unknown-linux-gnu/release/libpm_napi.so",
        bag: "packing-linux-x64",
        fileName: "pm.linux-x64-gnu.node",
    },
    {
        hostKey: "linux-arm64",
        target: "aarch64-unknown-linux-gnu",
        cargoRel: "target/aarch64-unknown-linux-gnu/release/libpm_napi.so",
        bag: "packing-linux-arm64",
        fileName: "pm.linux-arm64-gnu.node",
    },
    {
        hostKey: "darwin-x64",
        target: "x86_64-apple-darwin",
        cargoRel: "target/x86_64-apple-darwin/release/libpm_napi.dylib",
        bag: "packing-darwin-x64",
        fileName: "pm.darwin-x64.node",
    },
    {
        hostKey: "darwin-arm64",
        target: "aarch64-apple-darwin",
        cargoRel: "target/aarch64-apple-darwin/release/libpm_napi.dylib",
        bag: "packing-darwin-arm64",
        fileName: "pm.darwin-arm64.node",
    },
];

const copyOnly = process.argv.includes("--copy-only");
const targetArg = process.argv.find((arg, i) => process.argv[i - 1] === "--target");

function hostKey() {
    return `${process.platform}-${process.arch}`;
}

function copyOne(entry) {
    const src = join(root, entry.cargoRel);
    if (!existsSync(src)) {
        return false;
    }
    const libDir = join(packagesDir, entry.bag, "lib");
    mkdirSync(libDir, { recursive: true });
    for (const existing of readdirSync(libDir).filter((name) => name.endsWith(".node"))) {
        unlinkSync(join(libDir, existing));
    }
    const dest = join(libDir, entry.fileName);
    copyFileSync(src, dest);
    console.log(`copied ${entry.cargoRel} → ${dest}`);
    return true;
}

function buildOne(entry) {
    const args = ["build", "-p", "pm-napi", "--release"];
    if (entry.target) {
        args.push("--target", entry.target);
    }
    execSync(`cargo ${args.join(" ")}`, { cwd: root, stdio: "inherit", shell: true });
}

if (copyOnly) {
    let copied = 0;
    for (const entry of ARTIFACTS) {
        if (copyOne(entry)) {
            copied += 1;
        }
    }
    if (copied === 0) {
        console.error("no pm-napi artifacts found under target/");
        process.exit(1);
    }
    process.exit(0);
}

const entry =
    (targetArg ? ARTIFACTS.find((item) => item.target === targetArg) : undefined) ??
    ARTIFACTS.find((item) => item.hostKey === hostKey());

if (!entry) {
    console.error(`unsupported host ${hostKey()} (pass --target <triple> or --copy-only)`);
    process.exit(1);
}

buildOne(entry);
if (!copyOne(entry)) {
    console.error(`build finished but artifact missing: ${entry.cargoRel}`);
    process.exit(1);
}
