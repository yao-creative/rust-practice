Your intent is **incremental modular-design practice**: start with trivial crate composition, then progressively strengthen the **compile-time contracts** between crates.

I'd use a sequence where each step introduces exactly **one new architectural constraint**.

## 0. Baseline — two crates

```text
app → greeter
```

**Spec**

* `greeter` is a library crate.
* `app` is a binary crate.
* `app` may depend on `greeter`.
* `greeter` must not depend on `app`.
* `greeter` exposes exactly one public function: `greet`.

**Goal:** understand Cargo dependency edges.

---

## 1. Add a second library

```text
app → formatter
app → greeter
```

**Spec**

* `formatter` contains formatting logic.
* `greeter` contains greeting logic.
* `app` composes them.
* Neither library may depend on `app`.

**Goal:** understand **horizontal composition**.

Then deliberately refactor to:

```text
app → formatter → greeter
```

**Question:** what API does `formatter` expose so that `app` doesn't need to know about `greeter`?

This teaches **dependency hiding**.

---

# 2. Establish a public/private boundary

Inside `greeter`:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello {}", internal_normalize(name))
}

fn internal_normalize(name: &str) -> String {
    name.trim().to_string()
}
```

**Spec**

* `greet` is public.
* `internal_normalize` is private.
* `app` must not be able to call `internal_normalize`.

Now your contract is:

$$
API_{greeter} \subset Implementation_{greeter}
$$

The compiler enforces this.

**Goal:** learn that Rust visibility is part of your architectural boundary.

---

# 3. Replace primitive arguments with domain types

Instead of:

```rust
pub fn greet(name: &str)
```

introduce:

```rust
pub struct Name(String);

pub fn greet(name: Name) -> String
```

Then decide how `Name` can be constructed.

For example:

```rust
impl Name {
    pub fn new(value: String) -> Option<Self> {
        if value.trim().is_empty() {
            None
        } else {
            Some(Self(value))
        }
    }
}
```

Now the crate contract becomes:

$$
Name = {x \in String \mid valid(x)}
$$

Rather than allowing every `String` to masquerade as a valid name.

**Goal:** use the type system to enforce invariants.

---

# 4. Introduce a trait boundary

Create:

```text
greeter
    ↓
Greeter trait
    ↓
app
```

For example:

```rust
pub trait Greeter {
    fn greet(&self, name: &Name) -> String;
}
```

and:

```rust
pub struct SimpleGreeter;

impl Greeter for SimpleGreeter {
    ...
}
```

**Spec**

* `Greeter` defines the capability.
* `SimpleGreeter` implements it.
* Consumers depend on `Greeter`, not `SimpleGreeter`.

Now you have:

$$
Implementation \models Contract
$$

This is your first genuine **substitution boundary**.

Don't do this automatically in every crate, though. The exercise is to understand *why* you'd introduce the trait.

---

# 5. Separate contract from implementation

Now create three crates:

```text
contracts
implementation
app
```

with:

```text
app → contracts
app → implementation
implementation → contracts
```

`contracts` contains:

```rust
pub trait Greeter {
    fn greet(&self, name: &Name) -> String;
}
```

`implementation` contains:

```rust
pub struct SimpleGreeter;

impl Greeter for SimpleGreeter {
    ...
}
```

`app` composes them.

Your dependency graph becomes:

$$
\begin{aligned}
implementation &\rightarrow contracts \
app &\rightarrow contracts \
app &\rightarrow implementation
\end{aligned}
$$

The important property is:

$$
contracts \not\rightarrow implementation
$$

**Goal:** understand the difference between **API ownership** and **implementation ownership**.

---

# 6. Make the build enforce the architecture

Now introduce a deliberate violation.

Suppose:

```text
domain → implementation
```

is forbidden.

You want the build to fail if somebody accidentally adds that dependency.

At this stage, introduce a dependency-checking tool such as `cargo-deny` or a dependency graph check.

The specification becomes:

> `domain` must have no dependency on `app`, `runtime`, or infrastructure crates.

Now the architecture is no longer merely documented.

It is **executable as a build constraint**.

Think of it as:

$$
Build : Source \rightarrow
\begin{cases}
Success & \text{if architecture constraints hold}\
Failure & \text{otherwise}
\end{cases}
$$

This is a major transition.

---

# 7. Workspace-level feature contracts

Now create:

```text
core
backend
cli
```

and give `core` zero optional dependencies.

Then introduce Cargo features deliberately.

For example:

```text
core
├── default
└── serde
```

**Spec**

* Default build must not require `serde`.
* `serde` support must be opt-in.
* Core functionality must compile without it.

Test both:

```bash
cargo check -p core
cargo check -p core --features serde
```

Now your crate has a **configuration contract**:

$$
Configuration \rightarrow Valid\ Build
$$

rather than only:

$$
Source \rightarrow Build
$$

---

# 8. Add compile-time contract tests

Create tests specifically for the public API.

For example, make an integration test:

```text
greeter/
├── src/
│   └── lib.rs
└── tests/
    └── api.rs
```

The test can only access `pub` items.

This gives you a useful property:

$$
Test_{external}(crate)
\subseteq
API_{public}(crate)
$$

So your integration tests naturally verify the **consumer-visible contract**, rather than implementation details.

---

# 9. Add type-level contracts

Now make illegal states harder to construct.

Instead of:

```rust
struct User {
    id: String,
}
```

introduce:

```rust
struct UserId(u64);
```

and make constructors enforce validity.

The progression is:

```text
String
  ↓
UserId
  ↓
Validated UserId
```

You're moving invariants from:

```text
runtime convention
```

toward:

```text
compile-time structure
```

Not everything can be compile-time enforced, but push invariants into types where it is genuinely useful.

---

# 10. Final tiny project

By the end, I'd have you build only this:

```text
mini-system/
│
├── Cargo.toml
│
└── crates/
    ├── domain/
    ├── contracts/
    ├── implementation/
    └── app/
```

With the intended dependency lattice:

$$
\require{AMScd}
\begin{CD}
&& app &&\
& \swarrow && \searrow &\
contracts &&&& implementation\
& \nwarrow && \nearrow &\
&& domain &&
\end{CD}
$$

Conceptually:

$$
domain \rightarrow contracts
$$

$$
implementation \rightarrow contracts
$$

$$
app \rightarrow {contracts, implementation}
$$

and **nothing points upward into `app`**.

The important thing is that you incrementally acquire mechanisms for enforcing this:

| Stage | Contract mechanism         |
| ----- | -------------------------- |
| 0     | Cargo dependency graph     |
| 1     | Crate separation           |
| 2     | `pub` / private visibility |
| 3     | Newtypes + constructors    |
| 4     | Traits                     |
| 5     | Contract crates            |
| 6     | Dependency/build checks    |
| 7     | Cargo features             |
| 8     | Integration tests          |
| 9     | Type-level invariants      |

### The principle to practice

For every new module/crate, ask these four questions:

1. **What set of concepts does this crate own?**
2. **What does it expose as its public contract?**
3. **Which other crates is it allowed to know about?**
4. **Which of those rules does Cargo/the Rust compiler actually enforce?**

That last question is the most important.

You don't want:

> "We have a modular architecture because the developers remember not to violate it."

You want progressively stronger:

$$
Documentation
;<;
Convention
;<;
Tests
;<;
Build\ Constraints
;<;
Type\ System
$$

where each step moves an architectural invariant closer to something the machine **cannot accidentally violate**.
