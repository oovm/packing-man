# crates

装箱研究 Rust 算法栈（`pm-*`）。 **crate 名一律名词**。问题分类（Birgin 四族）见
`规划设计/packing-man/00-问题分类与crates分层.md`。

| crate                                      | 说明                                         |
|--------------------------------------------|----------------------------------------------|
| [`pm-types`](pm-types/readme.md)           | `ProblemFamily`、Problem、Solution、SolverId |
| [`pm-geometry`](pm-geometry/readme.md)     | 圆/矩形/凸域几何谓词                         |
| [`pm-solver-cpu`](pm-solver-cpu/readme.md) | CPU 贪心 / 力松弛                            |
| [`pm-solver-gpu`](pm-solver-gpu/readme.md) | GPU 力松弛（feature `gpu`）                  |
| [`pm-solver`](pm-solver/readme.md)         | 求解注册表                                   |
| [`pm-svg`](pm-svg/readme.md)               | SVG 可视化                                   |
| [`pm-view`](pm-view/readme.md)             | CLI                                          |
| [`pm-wasm`](pm-wasm/readme.md)             | Wasm cdylib                                  |
| [`pm-napi`](pm-napi/readme.md)             | Node N-API                                   |

```bash
cargo check --workspace
cargo run -p pm-view -- out.svg --count 19
```
