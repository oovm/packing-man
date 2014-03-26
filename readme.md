# Packing Man

几何排样研究（Birgin 四族 + Packomania 实例）。科学计算壳：`@sxo/packing` + VMZ 演示。crate 前缀 **pm-**。

## 快速开始

```bash
cargo check --workspace
pnpm run build:napi
pnpm run build:ts
pnpm --filter @sxo/packing exec packing result.svg --container-r 10 --circle-r 1 --count 19 --algorithm force

# 断点续跑（力松弛 / 可变半径）
node projects/packages/packing/bin/packing.mjs out.svg --algorithm force --checkpoint-out run.cp.json --iter-budget 200
node projects/packages/packing/bin/packing.mjs out.svg --algorithm force --checkpoint-in run.cp.json --checkpoint-out run.cp.json --iter-budget 400
```

## Crate 分层

```text
pm-types          ← 问题分类合同（Birgin 四族）
pm-geometry       ← 几何谓词（circle / orthogonal / convex / sphere）
pm-solver         ← `SolverArch` trait（CPU/GPU）+ 算法 + 调度
pm-svg            ← Solution → SVG
pm-wasm / pm-napi ← Wasm / Node 绑定
@sxo/packing      ← CLI（`packing` 命令）
```

## 仓库布局

```text
packing-man/
  projects/
    crates/
    packages/
      packing/                 # @sxo/packing
      packing-unknown-wasm32/
      packing-<triple>/
      homepage/
  scripts/
    build-wasm.mjs
    build-napi.mjs
```

## Web 演示

```bash
pnpm install
pnpm run build:wasm
pnpm run build:ts
pnpm run dev:homepage
```

设计文档见工作区 `规划设计/packing-man/`。
