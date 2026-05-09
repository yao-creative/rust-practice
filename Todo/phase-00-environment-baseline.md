# Phase 0 (Week 0): Environment Baseline

## Objectives
- Establish reproducible tooling and CI guardrails.

## Exercises
- [x] Install Rust toolchain with `rustup`.
- [x] Add components: `clippy`, `rustfmt`.
- [x] Create a workspace with crates: `core`, `app`, `integration_tests`.
- [x] Add CI checks for:
  - [x] `cargo fmt --all -- --check`
  - [x] `cargo clippy --all-targets --all-features -- -D warnings`
  - [x] `cargo test --all`

## Done When
- [x] CI enforces quality gates on every commit.

## Evidence
- PR:
- CI run: `.github/workflows/ci.yml` (`push` on all branches + `pull_request`).
- Notes: Workspace root `Cargo.toml` with members `crates/core`, `crates/app`, `crates/integration_tests`. Local validation run completed for `fmt`, `clippy`, and `test`.
