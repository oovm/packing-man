# pm-wasm

`pm_solve_json` / `pm_render_svg` Wasm 导出。

Wasm 产物由 `scripts/build-wasm.mjs` 构建并拷贝到 `@sxo/packing-unknown-wasm32`。  
**不在 Rust 侧用 wasmtime 做集成测试**——宿主验证走 Node（`@sxo/packing`）或浏览器（homepage）。

```bash
pnpm run build:wasm
pnpm run build:ts
pnpm run dev:homepage
```
