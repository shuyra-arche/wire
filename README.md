# wire

`wire` 是 shuyra 的 `no_std` 字节线协议底座：定义定宽字段与变长游标的编解码、
有界消息与请求/回复契约，以及尾段字节和同族项的读写。

## 独立构建与测试

本目录是独立 Cargo 工作区，不依赖其他本地路径。工具链由
`rust-toolchain.toml` 固定为 `nightly-2026-09-04`。

RISC-V 库构建：

```sh
cd wire
cargo build --target riscv64gc-unknown-none-elf --locked
```

宿主测试（测试本体在独立工作区，必须显式给出 manifest）：

```sh
cargo test --manifest-path tests/host/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked
```

依赖已缓存后，同一组命令可离线复验：

```sh
cargo build --target riscv64gc-unknown-none-elf --locked --offline
cargo test --manifest-path tests/host/Cargo.toml \
  --target x86_64-unknown-linux-gnu --locked --offline
```

`wire` 库自身关闭了 `test`、`bench` 与 `doctest`；真实测试只在
`tests/host` 中运行。该测试工作区通过 path 依赖使用本 crate，并在自己的
`Cargo.lock` 中固定公开的 `mold` 组件
（`https://github.com/shuyra-arche/mold.git`，
rev `f52501f4c91b686eded8b4bcd743b4e96084b7f9`）。测试不涉及 QEMU。
