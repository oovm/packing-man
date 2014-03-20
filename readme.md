# Packing Man

几何排样研究（Birgin 四族 + Packomania 实例）。科学计算壳：`@sxo/packing` + VMZ 演示。crate 前缀 **pm-**。

## 快速开始

```bash
cargo check --workspace
cargo run -p pm-view -- result.svg --container-r 10 --circle-r 1 --count 19 --algorithm force
```

## Crate 分层

```text
pm-types          ← 问题分类合同（Birgin 四族）
pm-geometry       ← 几何谓词（circle / orthogonal / convex / sphere）
pm-solver-cpu     ← CPU 算法
pm-solver-gpu     ← GPU 算法（feature gpu）
pm-solver         ← 注册表 + 调度
pm-svg            ← Solution → SVG
pm-view           ← CLI
pm-wasm / pm-napi ← Wasm / Node 绑定
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
