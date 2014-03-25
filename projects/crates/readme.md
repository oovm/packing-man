# crates

装箱研究 Rust 算法栈（`pm-*`）。 **crate 名一律名词**。问题分类（Birgin 四族）见
`规划设计/packing-man/00-问题分类与crates分层.md`。

| crate                                      | 说明                                         |
|--------------------------------------------|----------------------------------------------|
| [`pm-types`](pm-types/readme.md)           | `ProblemFamily`、Problem、Solution、SolverId |
| [`pm-geometry`](pm-geometry/readme.md)     | 圆/矩形/凸域几何谓词                         |
| [`pm-checkpoint`](pm-checkpoint/readme.md) | 迭代断点 JSON + 续跑                         |
| [`pm-solver`](pm-solver/readme.md)         | `SolverArch` + 算法实现 + 注册表             |
| [`pm-svg`](pm-svg/readme.md)               | SVG 可视化                                   |
| [`pm-benchmark`](pm-benchmark/readme.md)   | 分类基准 fixtures + 回归                     |
| [`pm-wasm`](pm-wasm/readme.md)             | Wasm cdylib                                  |
| [`pm-napi`](pm-napi/readme.md)             | Node N-API                                   |

```bash
cargo check --workspace
pnpm --filter @sxo/packing exec packing out.svg --count 19
```
