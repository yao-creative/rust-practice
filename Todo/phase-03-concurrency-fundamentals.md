# Phase 3 (Weeks 5-6): Concurrency Fundamentals

## Objectives
- Implement correct, testable multithreaded behavior.

## Exercises
- [ ] Build a multithreaded job executor.
- [ ] Implement graceful shutdown semantics.
- [ ] Build version A: channel-driven architecture.
- [ ] Build version B: shared state with `Arc<Mutex<_>>`.
- [ ] Compare complexity/perf/maintainability of A vs B.
- [ ] Add deterministic tests for race-prone paths.
- [ ] Add stress tests for shutdown and queue pressure.

## Done When
- [ ] No deadlocks under stress tests.
- [ ] Shutdown behavior is predictable and clean.

## Evidence
- PR:
- Stress test output:
- Comparison notes:
