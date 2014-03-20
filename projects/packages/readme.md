# packages

| 目录                                                         | 包名                          | 说明                               |
|--------------------------------------------------------------|-------------------------------|------------------------------------|
| [`packing`](packing/readme.md)                               | `@sxo/packing`                | 宿主 TS（`src/wasm` + `src/node`） |
| [`packing-unknown-wasm32`](packing-unknown-wasm32/readme.md) | `@sxo/packing-unknown-wasm32` | wasm 平台袋                        |
| [`packing-win32-x64`](packing-win32-x64/readme.md)           | `@sxo/packing-win32-x64`      | native 平台袋                      |
| [`packing-win32-arm64`](packing-win32-arm64/readme.md)       | `@sxo/packing-win32-arm64`    | native 平台袋                      |
| [`packing-darwin-x64`](packing-darwin-x64/readme.md)         | `@sxo/packing-darwin-x64`     | native 平台袋                      |
| [`packing-darwin-arm64`](packing-darwin-arm64/readme.md)     | `@sxo/packing-darwin-arm64`   | native 平台袋                      |
| [`packing-linux-x64`](packing-linux-x64/readme.md)           | `@sxo/packing-linux-x64`      | native 平台袋                      |
| [`packing-linux-arm64`](packing-linux-arm64/readme.md)       | `@sxo/packing-linux-arm64`    | native 平台袋                      |
| [`homepage`](homepage/readme.md)                             | `@sxo/packing-homepage`       | VMZ 演示（仅依赖 wasm 袋）         |

Rust 绑定：`../crates/pm-wasm`、`../crates/pm-napi`。
