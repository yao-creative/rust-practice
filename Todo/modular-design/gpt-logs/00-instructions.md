Yes. Since you already did one `cargo new`, the simplest approach is to turn the project into a **Cargo workspace** and create the second crate inside it.

### 1. Suppose you currently have

```text
my-project/
├── Cargo.toml
└── src/
    └── main.rs
```

If this is currently your `app`, rename/move it into:

```text
my-project/
├── Cargo.toml          # workspace
└── crates/
    ├── app/
    │   ├── Cargo.toml
    │   └── src/
    │       └── main.rs
    └── greeter/
        ├── Cargo.toml
        └── src/
            └── lib.rs
```

But there's an even easier way if you're willing to restart the tiny exercise.

### 2. Start from an empty directory

```bash
mkdir mini-project
cd mini-project

cargo new app
cargo new greeter --lib
```

You now have:

```text
mini-project/
├── app/
│   ├── Cargo.toml
│   └── src/main.rs
└── greeter/
    ├── Cargo.toml
    └── src/lib.rs
```

### 3. Make them a workspace

Create `mini-project/Cargo.toml`:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

The individual crates keep their own `Cargo.toml`.

### 4. Tell `app` that it depends on `greeter`

In `app/Cargo.toml`:

```toml
[package]
name = "app"
version = "0.1.0"
edition = "2024"

[dependencies]
greeter = { path = "../greeter" }
```

The important line is:

```toml
greeter = { path = "../greeter" }
```

That creates the Cargo dependency edge:

$$
app \longrightarrow greeter
$$

### 5. Put something in `greeter`

`greeter/src/lib.rs`:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}
```

### 6. Import it from `app`

`app/src/main.rs`:

```rust
use greeter::greet;

fn main() {
    println!("{}", greet("Yi"));
}
```

### 7. Run it

From `mini-project/`:

```bash
cargo run -p app
```

Result:

```text
Hello, Yi!
```

That's the entire composition mechanism.

The conceptual model is:

$$
\texttt{greeter/src/lib.rs}
\xrightarrow{\text{Cargo package}}
\texttt{greeter}
\xrightarrow{\text{dependency}}
\texttt{app}
$$

And importantly, **you don't use `mod greeter;`**. `mod` is for modules *inside a crate*. A Cargo dependency is a separate **crate**.

So for your learning exercise, I'd explicitly use:

```bash
cargo new app
cargo new greeter --lib
```

then experiment by adding/removing this one line:

```toml
greeter = { path = "../greeter" }
```

That will make the crate-composition boundary very concrete.
