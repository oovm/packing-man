#!/usr/bin/env python3
"""One-shot scaffold: copy homepage sources and blackhole vendor into zhihu-playground."""

from __future__ import annotations

import os
import shutil
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[2]
HOMEPAGE = REPO / "projects" / "packages" / "homepage"
BLACKHOLE = REPO / "projects" / "packages" / "blackhole"
VMZ_MANIFEST = REPO / "scripts" / "vmz" / "host-runtime-files.json"

SKIP_NAMES = {"node_modules", "dist", ".git"}


def copy_tree(src: Path, dst: Path) -> None:
    if not src.exists():
        raise FileNotFoundError(src)
    for dirpath, dirnames, filenames in os.walk(src):
        dirnames[:] = [d for d in dirnames if d not in SKIP_NAMES]
        rel = Path(dirpath).relative_to(src)
        out_dir = dst / rel
        out_dir.mkdir(parents=True, exist_ok=True)
        for name in filenames:
            shutil.copy2(Path(dirpath) / name, out_dir / name)


def main() -> None:
    (ROOT / "src").mkdir(parents=True, exist_ok=True)
    copy_tree(HOMEPAGE / "src", ROOT / "src")

    scripts = ROOT / "scripts"
    scripts.mkdir(parents=True, exist_ok=True)
    shutil.copy2(HOMEPAGE / "scripts" / "copy-wasm.mjs", scripts / "copy-wasm.mjs")
    (scripts / "vmz").mkdir(parents=True, exist_ok=True)
    shutil.copy2(VMZ_MANIFEST, scripts / "vmz" / "host-runtime-files.json")

    public = ROOT / "public"
    public.mkdir(parents=True, exist_ok=True)
    if (HOMEPAGE / "public").exists():
        for item in (HOMEPAGE / "public").iterdir():
            target = public / item.name
            if item.is_dir():
                shutil.copytree(item, target, dirs_exist_ok=True)
            else:
                shutil.copy2(item, target)

    vendor = ROOT / "vendor" / "blackhole"
    vendor.mkdir(parents=True, exist_ok=True)
    for rel in ("package.json", "tsconfig.json"):
        shutil.copy2(BLACKHOLE / rel, vendor / rel)
    copy_tree(BLACKHOLE / "dist", vendor / "dist")
    (vendor / "lib").mkdir(parents=True, exist_ok=True)
    shutil.copy2(BLACKHOLE / "lib" / "pm_wasm_bg.wasm", vendor / "lib" / "pm_wasm_bg.wasm")

    readme_src = HOMEPAGE / "readme.md"
    if readme_src.exists():
        shutil.copy2(readme_src, ROOT / "readme.md")

    print(f"scaffolded {ROOT}")


if __name__ == "__main__":
    main()
