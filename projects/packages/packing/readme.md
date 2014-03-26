# @sxo/packing

Packing-Man 宿主：`/wasm` 与 `/node` 加载 `pm-wasm` / `pm-napi`，JSON 求解 + SVG 渲染。

CLI `packing` 支持 `--checkpoint-in` / `--checkpoint-out` / `--iter-budget` 从上次布局继续迭代优化。

## CLI

```bash
pnpm exec packing --list-fixtures
pnpm exec packing --fixture circle_sphere_packing/equal_circles_in_circle/n19 --out out.svg
pnpm exec packing result.svg --container-r 10 --circle-r 1 --count 19 --algorithm force
```
