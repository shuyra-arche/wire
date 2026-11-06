# wire

Shuyra 的 `no_std` 字节协议兼容门面。`src/lib.rs` 重导出
[Kernel](https://github.com/shuyra-arche/kernel) 的 `kernel-api-wire`，保持原有
`Contract`、`Message`、`Field`、`Span` 及模块路径的类型身份。
新接口定义在 Kernel 中维护，本仓不复制编解码实现。

根库禁用内置测试；`tests/host` 是独立工作区，验证协议行为与规范类型身份。

## 工具链与维护

使用 `rust-toolchain.toml` 固定的 `nightly-2026-09-04`，首次运行需要 Rustup
下载工具链及 RV64 目标。依赖版本以 Cargo 清单和锁文件为准。
下列命令从仓库根目录执行；宿主测试不代表真实内核运行验证。

```sh
cargo test --manifest-path tests/host/Cargo.toml --target x86_64-unknown-linux-gnu --locked
cargo build --lib --target riscv64gc-unknown-none-elf --locked
```

## 贡献与来源

维护与 PR 规则见 [Shuyra 贡献规范](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md)。
来源、作者与许可情况见 [PROVENANCE.md](PROVENANCE.md)。
