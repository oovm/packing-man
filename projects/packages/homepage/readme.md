# @sxo/packing-homepage

Packing-Man **VMZ 多页演示站**（`@sxo/packing/wasm`）。

| 路由 | 页面 |
|------|------|
| `/` | 首页概览 |
| `/demo` | 交互求解（支持 `?fixture=`） |
| `/fixtures` | 基准实例目录 |
| `/about` | 架构说明 |

导航使用 VMZ `data-vmz-route` 客户端路由；`public/fixtures/` 与 `pm-benchmark` 同步。

## 开发

```bash
pnpm --filter @sxo/packing-unknown-wasm32 run build:wasm
pnpm --filter @sxo/packing run build
pnpm --filter @sxo/packing-homepage run dev
```

或仓库根目录：`pnpm run dev:homepage`
