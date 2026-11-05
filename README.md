# wire

`wire` 是 `no_std` 字节协议的兼容门面。编解码定义由 Kernel 的
[`kernel-api-wire`](https://github.com/shuyra-arche/kernel) 唯一持有；本仓
`src/lib.rs` 通过 `pub use kernel_api_wire::*;` 保留原有路径与类型身份。

根库和宿主测试统一固定 Kernel 完整版本：
`90d43dbf3ee43fd146f487718eb39e88f2507c05`。

库构建只有 `wire -> kernel-api-wire` 一条依赖。独立的 `tests/host` 工作区
另以 `mold` 为别名使用同版本的 `kernel-api-macros`，并直接依赖规范
`kernel-api-wire` 验证类型身份。两个工作区的锁文件均入库，不使用本地依赖补丁。

本次只推进上述依赖版本和锁文件；门面源码、行为测试与身份测试保持不变。
其他消费者及产品的版本推进由各自仓库完成。早期提取与门面迁移的记录见
[PROVENANCE.md](PROVENANCE.md)。

## 验证

库自身关闭了 `test`、`bench` 与 `doctest`。真实测试位于独立工作区：

```sh
cargo test --manifest-path tests/host/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
```

本次使用环境中已安装的 `nightly-2026-10-09`，显式指定其 Cargo、rustc 和
rustdoc 执行上述命令。宿主测试 8 项通过（5 项行为测试、3 项类型身份测试），
RV64 库构建通过。依赖元数据确认每个图只有一个 `kernel-api-wire` 定义来源，
宿主宏也来自相同 Kernel 版本；既有 registry 包版本保持不变。

`rust-toolchain.toml` 仍声明 `nightly-2026-09-04`；本次未使用该声明版本验证。
测试不涉及 QEMU。
