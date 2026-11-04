# wire

`wire` 是 shuyra `no_std` 字节线协议的**兼容门面**：编解码正文只有一份，住 Kernel 仓的
`kernel-api-wire`；本仓只把它公开面照原样重导出（`pub use kernel_api_wire::*;`），
不再持有第二份定义。

## 唯一 owner 与精确 SHA

- owner 包：`kernel-api-wire`，仓 <https://github.com/shuyra-arche/kernel.git>
- 固定 rev：`a9986c82127cf25183bf5c30e7d42e4a9376b734`（Kernel 全 SHA；运行期与测试期同一条）

本阶段删除了三份重复正文 `src/contract.rs`、`src/field.rs`、`src/message.rs`：它们的字节
原样留在提取阶段的 git 历史里（见 `PROVENANCE.md`），Kernel 侧那份就是唯一 owner。
`src/lib.rs` 因此只剩门面：`Contract`、`Message`、`Field`、`Span`、尾巴那几只手
（`store_tail` / `fetch_tail` / `store_bytes` / `fetch_bytes` / `total` / `times`）与 `OK`，
以及 `contract` / `field` / `message` 三个模块路径都随重导出保留，既有调用点路径不动。

## 依赖图：运行期 vs 开发期

运行期（库构建）只有一条边：

```text
wire -> kernel-api-wire (git, rev = a9986c82…)
```

宿主测试工作区（`tests/host`）另有两格：

- 运行期 `wire = { path = "../.." }`（门面本身）与
  `mold = { package = "kernel-api-macros", git = …, rev = a9986c82… }`：`mold` 只是
  `#[derive(Frame)]` 的来源名，实际包是 Kernel 仓的 `kernel-api-macros`，**不是**原
  `https://github.com/shuyra-arche/mold.git` 那一枚。
- test-only 的 `kernel-api-wire`（同一 SHA）只在 `[dev-dependencies]` 里，供身份夹具直接点名规范包。

故运行期图里没有任何原 `env` / `wire` / `mold` 包；原 `mold` 仓只作为 baseline 留在生产图外。

## 行为不变

- `tests/host/lib.rs` 五条既有夹具源码字节未改：默认 `::env::wire` 与显式
  `codec = ::wire` 的派生产出同样字节、`Field` 的宽度/端序/越界、变长 UTF-8 帧的往返与截断拒绝、
  字符串长度字节的 255/256 界、条数与长度算术溢出。
- 新增 `tests/host/tests/identity.rs`：每个探针只在**一侧**写泛型上界、正文调用**另一侧**的同名手
  （两个方向各一份），证明 `Field`、`Span`、`Message`、`Contract` 及其 `Buf` / `Request` /
  `Response` 关联类型都是同一枚；另有一条夹具让自定义包**只实现门面的**
  `Message`/`Contract`，再要求它满足规范包上界。
- 字节/UTF-8/截断那几条行为断言不重复搬进身份夹具：它们仍由上面五条既有夹具原样承担。

## 独立构建与测试

本目录是独立 Cargo 工作区，不依赖其他本地路径。工具链由 `rust-toolchain.toml` 固定为
`nightly-2026-09-04`。以下命令由 Root 运行（复验结果见下）；本 worker 未运行 Cargo。

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

Root 复验结果：在**隔离安装的声明工具链** `nightly-2026-09-04`
（`rustc 1.100.0-nightly (a69a63265)`）下复跑 `--locked` 宿主测试 8 条（5 条既有夹具 +
3 条身份夹具）与 RISC-V 库构建（`cargo build --lib --target riscv64gc-unknown-none-elf --locked`），
两者均通过；元数据里只剩固定同一 SHA 的 `kernel-api-wire` / `kernel-api-macros`，没有任何原
`env` / `wire` / `mold` owner。更早一次复验用的是环境覆盖
`RUSTUP_TOOLCHAIN=nightly-2026-10-09`，只作历史记录——当前已无"声明 pin 未被验证"的缺口。
`rust-toolchain.toml` 仍声明 `nightly-2026-09-04`，本次未改动它。

`wire` 库自身关闭了 `test`、`bench` 与 `doctest`；真实测试只在 `tests/host` 中运行。
该测试工作区通过 path 依赖使用本 crate，并在自己的 `Cargo.lock` 中固定上面那两枚 Kernel 包；
测试不涉及 QEMU。两份 `Cargo.lock`（根与 `tests/host`）都已按门面图重新生成并入库，
既有 registry 版本按原样保留。`tests/host/.gitignore` 保留自己的 `target/`，其 `Cargo.lock`
不被忽略，故 fixture 自带独立锁。

## 同步门（本 PR 不做生产切换）

这三个门必须**同一 SHA 原子推进**，缺一个就会出现"旧 owner 定义 + 新 owner 定义"并存，
两份同名 trait 会让类型身份断掉：

1. **env / wire 两个适配器**：本仓这一枚 `wire` 与 `env` 兼容门面都只重导出各自 owner
   （`kernel-api-wire` / `kernel-api-env`），并固定同一个全 SHA；
2. **mold 拆分**：proc-macro 不能既保留 `entry` 又直接重导出这批派生宏，故原 `mold` 仓的
   包装/拆分单独成一个 PR（普通门面 + 独立 entry owner），本阶段不声称 `mold` 替换完成；
3. **整个消费闭包**：上面的拆分落地后，所有消费者与锁文件一次性切到同一全 SHA。

本 PR 停在 staging：只交付 `wire` 这一枚兼容门面与其独立测试夹具，**不声称生产已迁移**，
不改任何生产消费者或依赖补丁。**本仓之外**没有任何消费者锁被改动；**本仓之内**两份门面生成锁
（`Cargo.lock` 与 `tests/host/Cargo.lock`）已按门面图重新生成并入库。owner 包已可由公开的
精确 Git SHA 取得，不涉及 registry 发布。
