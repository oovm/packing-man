# TON 黑洞可视化首页（VMZ）

基于 `@sxo/packing/wasm` 的 **VMZ** 演示页：史瓦西黑洞 CPU 光线追踪、UCF 运行时探针、L0 readback 上屏。`pm_wasm_bg.wasm` 由
`copy:wasm` 从平台袋复制到 `public/`。

依赖 npm 上的 `@vmz/vmz` / `@vmz/core`（无需本地 `vmz-framework` 路径）。

## 开发

```bash
pnpm --filter @sxo/packing-unknown-wasm32 run build:wasm
pnpm --filter @sxo/packing run build
pnpm --filter @sxo/packing-homepage run dev
```

浏览器打开后自动渲染；调整参数会实时重新计算。WebGPU 可用时走 **UCF L2**（`pm_l2_present_rgba8_webgpu`），否则 L0 readback。
