# Provenance

## Source

- Repository: <https://github.com/anreonyr/sqware>
- Commit: `6660f3d43a9d78b521542092da9ed891e36b38da`
- Branch at extraction: `codex/shuyra-wire-extraction`
- Extracted path: `crates/wire`

This directory is the source snapshot of that commit. The four Rust files under
`src/` and `tests/host/lib.rs` are byte-for-byte copies. `rust-toolchain.toml`
and `tests/host/Cargo.toml` are copied unchanged. The root `Cargo.toml` is a new
standalone manifest with explicit `version = "0.1.0"` and `edition = "2024"`;
the upstream manifest inherited both from its workspace. The root `Cargo.lock`
was generated for this dependency-free crate. The accepted host test lock was
copied from the source worktree, and its ignored `unicode-ident` remains pinned
to `1.0.24`.

`tests/host/.gitignore` keeps its original `target/` entry. Its single
`Cargo.lock` ignore line was removed so this component publishes its own test
lock; this is the only metadata change to the copied host test files.

## File mapping

| Upstream | Here |
| --- | --- |
| `crates/wire/src/contract.rs` | `src/contract.rs` |
| `crates/wire/src/field.rs` | `src/field.rs` |
| `crates/wire/src/lib.rs` | `src/lib.rs` |
| `crates/wire/src/message.rs` | `src/message.rs` |
| `crates/wire/tests/host/Cargo.toml` | `tests/host/Cargo.toml` |
| `crates/wire/tests/host/Cargo.lock` | `tests/host/Cargo.lock` |
| `crates/wire/tests/host/lib.rs` | `tests/host/lib.rs` |
| `crates/wire/tests/host/.gitignore` | `tests/host/.gitignore` |
| `rust-toolchain.toml` | `rust-toolchain.toml` |

The upstream `crates/wire/Cargo.toml` is intentionally not copied as-is; this
crate has its own explicit `version = "0.1.0"` and `edition = "2024"`.

## Attribution

- Commit author and committer: `anreonyr <anreonyr@gmail.com>`

## History

The original repository history is retained at the source URL above. This
extraction is a snapshot without a rewritten or synthesized history.

## Licensing

The source project declares no license. At commit
`6660f3d43a9d78b521542092da9ed891e36b38da` no tracked `LICENSE`, `COPYING`, or
`NOTICE` file matched the project. No license text or license metadata is added
by this extraction; do not assume a license that the source does not grant.
