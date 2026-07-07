//! Library surface for embedding rtk's filter core into other Rust programs
//! (e.g. coco-rs post-exec Bash output filtering).
//!
//! This file is additive: it introduces a `lib` target alongside the existing
//! `main.rs` binary without modifying any upstream source, so `git merge
//! upstream/<branch>` stays conflict-free. `main.rs` remains the authoritative
//! binary root and is unaffected.
//!
//! ## What is exposed
//!
//! The modules re-exported here are the pure text→text engine — the rewrite /
//! classify engine (`discover`), the declarative TOML filter registry plus the
//! never-worse guard, ANSI strip, truncate and code-strip filters (`core`),
//! the shared parser types (`parser`), and the custom-filter trust store
//! (`hooks::trust` / `hooks::integrity`).
//!
//! Library callers never enter the CLI `run()` lifecycles, so the SQLite
//! savings ledger (`core::tracking`) and the telemetry ping (`core::telemetry`)
//! compile but never execute for a lib consumer.
//!
//! ## What is NOT exposed (and why)
//!
//! `cmds` — the per-family formatters — is intentionally omitted. Compiling it
//! pulls `cmds::js::vitest_cmd`, whose `run_test` references the binary-only
//! clap `Commands` enum defined in `main.rs`. Re-exporting the family
//! formatters therefore requires decoupling that enum from `cmds`, which is a
//! non-additive change to an upstream-hot file; it is tracked separately and
//! gated on need.

pub mod core;
pub mod discover;
pub mod hooks;
pub mod parser;
