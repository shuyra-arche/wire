#![no_std]

//! `wire` 兼容门面：**定义只有一份**，住 `kernel-api-wire`。
//!
//! 本仓不再持有编解码正文，只把唯一那一份公开面照原样重导出——`Contract`、`Message`、
//! `Field`、`Span`、尾巴那几只手与 `OK`，以及 `contract` / `field` / `message` 三个模块路径，
//! 故既有调用点的路径与类型身份都不动。

pub use kernel_api_wire::*;
