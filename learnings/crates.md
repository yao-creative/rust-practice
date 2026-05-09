# Crates and Modular Split (Rust Workspace)

`Crate` is Rust's compilation and module boundary. Strong teams usually split crates by stable responsibility boundaries, not by folder type.

## Common modular split

1. `core` (or `domain`)
- Pure business rules, types, validation, and policies.
- Avoid HTTP/DB/cloud SDK dependencies.
- Most unit tests live here.

2. `app` / `service` / `api`
- Runtime wiring: CLI/HTTP handlers, config, startup, dependency injection.
- Calls into `core`.
- Can depend on infrastructure/adapters.

3. `infrastructure` (optional, often several crates)
- DB, queues, external APIs, storage.
- Implements traits/interfaces defined in `core` or `app`.

4. `integration_tests`
- Black-box or semi-black-box tests across crate boundaries.
- Uses realistic wiring (sometimes real DB/container in CI).

## Typical workspace shape

```toml
[workspace]
members = ["crates/core", "crates/app", "crates/integration_tests"]
resolver = "2"
```

## How teams decide boundaries

- Split when change cadence differs (domain stable, adapters volatile).
- Split when dependency direction must be enforced.
- Split when compile-time or ownership benefits are clear.
- Avoid over-splitting too early into many tiny crates.

## Testing design and syntax

1. Unit tests (inside the same module/file)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_total() {
        assert_eq!(add(2, 2), 4);
    }
}
```

2. Integration tests (`tests/*.rs` in a crate)

```rust
#[test]
fn core_add_smoke() {
    assert_eq!(core::add(2, 2), 4);
}
```

3. Async tests

```rust
#[tokio::test]
async fn fetches_profile() {
    // arrange / act / assert
}
```

4. Error-path assertions

```rust
#[test]
fn rejects_empty_title() {
    let err = create_todo("").unwrap_err();
    assert!(matches!(err, TodoError::EmptyTitle));
}
```

5. Table-driven style

```rust
#[test]
fn parses_priority_cases() {
    let cases = [("high", 3), ("low", 1)];
    for (input, expected) in cases {
        assert_eq!(parse_priority(input).unwrap(), expected);
    }
}
```

## Practical testing pyramid for this repo

1. `core`: highest test density (logic, edge cases, invariants).
2. `app`: contract and wiring behavior tests.
3. `integration_tests`: critical end-to-end flows only.

