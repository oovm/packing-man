# TON 黑洞可视化（知乎 AI Works）

基于 `@sxo/packing-unknown-wasm32` 的 **Vite** 静态演示页：史瓦西黑洞 CPU 光线追踪、UCF 运行时探针、L2/L0 上屏。

独立 npm 包（`vendor/` 内置 wasm 加载器），远端构建仅依赖 `vite build`，无需 VMZ 原生工具链。

## 开发

```bash
cd projects/packages/zhihu-playground
npm install
npm run dev
```

浏览器打开后点击「渲染一帧」。WebGPU 可用时走 **UCF L2**，否则 L0 readback。

## Vendor 同步

仓内更新 wasm 加载器后，在仓库根执行 `pnpm build:wasm` 与 `pnpm build:ts`，再在本目录运行：

```bash
node ./scripts/sync-vendor-runtime.mjs
```

产物写入 `vendor/packing-unknown-wasm32/runtime/`（避免 CloudBase ZIP 排除名为 `dist` 的目录），并把 `pm_wasm_bg.wasm` 复制到
`vendor/.../lib/` 与 `public/`。 **wasm 不进 git**（仓根 `*.wasm`），只在本机存在后打进部署 ZIP。

## CloudBase 交付

先完成上述 sync，再运行 `zhihu-ai-works-deploy-helper` 生成 `zhihu-playground.zip` 并上传。远端 `vite build` 依赖 ZIP
内已携带的 vendored `runtime/` 与 wasm 字节，不会在 Hosting 环境编译 Rust。
