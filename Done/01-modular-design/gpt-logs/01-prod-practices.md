Your intent is **crate/package architecture**: understand what a Rust library crate actually is, how it differs from a binary crate, and how Cargo turns `Cargo.toml` declarations into a dependency/build graph.

## 1. What is a `lib`?

A Rust **library crate** is a crate whose compiled artifact is intended to be **used by other crates**.

For example:

```bash
cargo new greeter --lib
```

creates:

```text
greeter/
├── Cargo.toml
└── src/
    └── lib.rs
```

`lib.rs` is the **crate root**.

```rust
pub fn greet(name: &str) -> String {
    format!("Hello {name}")
}
```

Another crate can then do:

```rust
use greeter::greet;
```

---

## 2. What is the non-`lib` version?

When you run:

```bash
cargo new app
```

Cargo creates a **binary crate**:

```text
app/
├── Cargo.toml
└── src/
    └── main.rs
```

`main.rs` is the crate root.

It produces an executable:

```text
app
```

and starts at:

```rust
fn main() {
    // program entry point
}
```

So the basic distinction is:

|                                | Library crate                | Binary crate                                 |
| ------------------------------ | ---------------------------- | -------------------------------------------- |
| Root                           | `src/lib.rs`                 | `src/main.rs`                                |
| Purpose                        | reusable code                | executable program                           |
| Entry point                    | none                         | `fn main()`                                  |
| Can other crates depend on it? | yes                          | generally not as a normal library dependency |
| Typical role                   | domain, algorithms, services | CLI, server, application                     |

Formally, think of the two as different kinds of artifacts:

$$
Lib : Source \rightarrow ReusableArtifact
$$

while:

$$
Bin : Source \rightarrow ExecutableArtifact
$$

---

# 3. The important distinction: package ≠ crate

This is where Cargo initially feels confusing.

A **package** is what `Cargo.toml` describes.

A package can contain:

* zero or one library crate
* one or more binary crates

For example:

```text
my_package/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

is **one package containing two crates**:

$$
Package = {LibraryCrate, BinaryCrate}
$$

The package's `Cargo.toml` describes the package and its dependencies.

---

# 4. How `Cargo.toml` creates dependency relationships

Suppose you have:

```text
mini/
├── Cargo.toml
├── app/
│   ├── Cargo.toml
│   └── src/main.rs
└── greeter/
    ├── Cargo.toml
    └── src/lib.rs
```

Your workspace root:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

This says:

$$
Workspace = {app, greeter}
$$

But it **doesn't say that `app` depends on `greeter`**.

That relationship is declared in `app/Cargo.toml`:

```toml
[dependencies]
greeter = { path = "../greeter" }
```

Now Cargo constructs the dependency edge:

$$
app \rightarrow greeter
$$

The `greeter` package's `Cargo.toml` does not need to mention `app`.

So:

```text
app/Cargo.toml
       │
       │ declares
       ▼
greeter
```

---

# 5. What actually happens when you compile?

You write:

```rust
use greeter::greet;
```

Cargo sees:

```toml
[dependencies]
greeter = { path = "../greeter" }
```

and resolves:

$$
app \rightarrow greeter
$$

Then Rustc effectively has to compile the dependency before compiling the consumer:

$$
greeter
\rightarrow
rustc
\rightarrow
libgreeter
\rightarrow
rustc(app)
\rightarrow
app
$$

Conceptually:

$$
Source_{greeter}
\xrightarrow{rustc}
Artifact_{greeter}
$$

then:

$$
Source_{app} + Artifact_{greeter}
\xrightarrow{rustc}
Artifact_{app}
$$

The `use greeter::greet` statement therefore isn't what *creates* the dependency.

**Cargo creates the crate dependency from `Cargo.toml`; `use` tells Rust what names from that dependency the source code wants to access.**

That's an important distinction.

---

# 6. Why have libraries at all?

Because you generally want:

$$
ReusableLogic \neq ProgramEntryPoint
$$

Imagine:

```text
my-system/
├── core/
├── database/
├── http/
└── cli/
```

You don't want your database implementation living inside `main.rs`.

Instead:

```text
cli
 ↓
core
 ↓
...
```

The executable is then a **composition root**.

For example:

```rust
fn main() {
    let database = PostgresDatabase::new();
    let service = Service::new(database);

    run(service);
}
```

The actual implementation lives in libraries.

This is one of the most useful production patterns in Rust.

---

# 7. Production practice: usually libraries + thin binaries

A very common structure is:

```text
project/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

where:

```rust
// lib.rs

pub mod domain;
pub mod service;
pub mod storage;
```

and:

```rust
// main.rs

use my_project::service::Service;

fn main() {
    let service = Service::new();
    service.run();
}
```

The architectural idea is:

$$
Binary = Composition + Execution
$$

while:

$$
Library = ReusableSystem
$$

This is especially useful because integration tests can consume your library exactly like an external crate would.

---

# 8. Multiple binaries

A package can also contain multiple binaries:

```text
src/
├── lib.rs
├── main.rs
└── bin/
    ├── server.rs
    └── migrate.rs
```

Now you have:

$$
Package =
{Library, Main, Server, Migrate}
$$

They can all reuse the same library.

```text
             lib
          ↙   ↓   ↘
       main server migrate
```

This is extremely useful for applications with:

* CLI
* server
* migration tool
* admin tool
* worker

all sharing the same core code.

---

# 9. When should you make a separate crate?

Don't turn every module into a crate.

Start with:

```text
crate
├── module
├── module
└── module
```

and create another crate when you want a **stronger dependency boundary**.

A module gives you roughly:

$$
Module \subset Crate
$$

while a crate gives you:

$$
Crate_i \not\equiv Crate_j
$$

with Cargo enforcing their dependency relationship.

This means a crate boundary is useful when you want to enforce:

* dependency direction
* API visibility
* independent compilation
* independent testing
* optional dependencies
* feature boundaries
* reusable components
* architectural layering

For your kernel project, this distinction will become very important.

---

# 10. A good progression for your practice project

Don't jump straight into 10 crates.

Use this progression:

### Stage A

```text
app
```

One binary crate.

### Stage B

```text
app
 ↓
greeter
```

One binary + one library.

### Stage C

```text
app
 ↓
service
 ↓
domain
```

Three crates.

### Stage D

```text
       app
      ↙   ↘
 service  infrastructure
    ↓
 domain
```

Now you're practicing architectural dependency control.

### Stage E

Eventually:

```text
                  app
              ↙    ↓    ↘
          runtime service adapters
              ↓     ↓      ↓
                   domain
```

That is much closer to what you're eventually trying to do with your kernel/runtime.

---

## The key formal distinction

There are three different relations you should keep separate:

**Cargo dependency:**

$$
A \rightarrow B
$$

means:

> crate/package `A` is allowed to use crate `B`.

**Rust module containment:**

$$
Module \subset Crate
$$

means:

> this module is part of the implementation of this crate.

**Rust name usage:**

$$
use\ B::x
$$

means:

> this source file wants the name `x` from `B`.

So:

```text
Cargo.toml
    │
    │ establishes
    ▼
crate dependency graph
    │
    ▼
Rust compiler
    │
    │ resolves
    ▼
use statements / types / functions
```

**Best production heuristic:** use **modules for ordinary internal organization**, **library crates for meaningful dependency boundaries**, and **binary crates as thin composition/entry points**. Don't create a crate merely because a folder feels conceptually separate. Create one when you want the compiler/Cargo to enforce the separation.
