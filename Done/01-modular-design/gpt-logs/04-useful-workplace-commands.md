Yes. **There isn't a special `cargo workspace` command you need.** A workspace is primarily configured in `Cargo.toml`.

For your tiny crate-composition exercise, do this.

### 1. Create the project directory

If you already have `app` and `greeter`:

```bash
mkdir mini
cd mini

cargo new app
cargo new greeter --lib
```

You now have:

```text
mini/
├── app/
└── greeter/
```

### 2. Create the workspace `Cargo.toml`

At `mini/Cargo.toml`:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

That's the workspace setup.

Your structure is now:

```text
mini/
├── Cargo.toml          ← workspace
├── app/
│   ├── Cargo.toml      ← package
│   └── src/main.rs     ← binary crate
└── greeter/
    ├── Cargo.toml      ← package
    └── src/lib.rs      ← library crate
```

### 3. Add the dependency

In `app/Cargo.toml`:

```toml
[dependencies]
greeter = { path = "../greeter" }
```

Now Cargo's dependency graph contains:

$$
app \rightarrow greeter
$$

### 4. Useful workspace commands

From `mini/`:

```bash
cargo check
```

Checks the whole workspace.

```bash
cargo build
```

Builds the workspace.

```bash
cargo run -p app
```

Runs specifically the `app` package.

```bash
cargo check -p greeter
```

Checks specifically `greeter`.

```bash
cargo tree
```

Shows the dependency graph.

For your learning exercise, **`cargo tree` is particularly useful** because you can modify `Cargo.toml` and immediately observe how the crate graph changes.

### One important thing

You **don't need** a workspace if you just have:

```text
app/
└── greeter/
```

You could simply have `app` depend on `greeter` by path.

The workspace becomes useful when you want Cargo to treat several packages as **one coordinated project**:

$$
Workspace = {Package_1,\ldots,Package_n}
$$

with a shared build context and dependency graph.

So for the modular-Rust exercise, I'd absolutely use a workspace, because you're specifically practicing **crate composition and dependency graphs**.
