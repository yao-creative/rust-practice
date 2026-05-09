# Phase 0 (Week 0): Environment Baseline

## Objectives
- Establish reproducible tooling and CI guardrails.

## Exercises
- [x] Install Rust toolchain with `rustup`.
- [x] Add components: `clippy`, `rustfmt`.
- [ ] Create a workspace with crates: `core`, `app`, `integration_tests`.
- [ ] Add CI checks for:
  - [ ] `cargo fmt --all -- --check`
  - [ ] `cargo clippy --all-targets --all-features -- -D warnings`
  - [ ] `cargo test --all`

## Done When
- [ ] CI enforces quality gates on every commit.

## Evidence
- PR:
- CI run:
- Notes:
