Your intent is **incremental crate-boundary training**: you've learned `app → greeter`, so the next step should introduce only **one new modularity concept at a time**.

I would follow this progression rather than jumping straight into a "real" architecture.

## Level 2 — split implementation from interface

Turn your project into:

```text
mini/
├── Cargo.toml
├── app/
├── greeter/
└── formatter/
```

Dependency target:

$$
app \rightarrow formatter
\rightarrow greeter
$$

### Specification

`greeter` owns:

```rust
pub fn greet(name: &str) -> String
```

`formatter` depends on `greeter` and exposes:

```rust
pub fn format_greeting(name: &str) -> String
```

Internally:

```rust
greet(name)
```

is used by `formatter`.

`app` **must not directly depend on `greeter`**.

So the Cargo graph must be:

$$
E = {(app,formatter),(formatter,greeter)}
$$

and specifically:

$$
(app,greeter) \notin E
$$

### What you're learning

**Dependency transitivity and information hiding.**

You're making `formatter` a genuine modular boundary instead of merely putting files in different folders.

---

# Level 3 — create a domain crate

Now change the graph:

$$
app \rightarrow service \rightarrow domain
$$

Create:

```text
domain/
service/
app/
```

`domain` contains only types:

```rust
pub struct Name(String);
```

No printing.

No filesystem.

No Cargo dependencies.

### Specification

The `domain` crate must satisfy:

$$
Dependencies(domain)=\varnothing
$$

This is your first **dependency purity constraint**.

The service can depend on domain:

$$
service \rightarrow domain
$$

but:

$$
domain \not\rightarrow service
$$

### What you're learning

**Dependency direction.**

You're no longer just asking:

> "Can I split this into crates?"

You're asking:

> "Which concepts are allowed to know about which other concepts?"

---

# Level 4 — introduce a trait

Now make the service depend on an abstraction.

Create:

```text
domain/
ports/
service/
app/
```

`ports`:

```rust
pub trait Greeter {
    fn greet(&self, name: &Name) -> String;
}
```

`service` consumes the trait.

An implementation lives elsewhere:

```text
simple_greeter/
```

The important dependency structure becomes something like:

$$
service \rightarrow ports
$$

$$
simple_greeter \rightarrow ports
$$

rather than:

$$
service \rightarrow simple_greeter
$$

This teaches you **dependency inversion**.

---

# Level 5 — composition root

Now make `app` responsible for selecting the implementation.

For example:

```rust
let greeter = SimpleGreeter::new();

let service = GreetingService::new(greeter);

service.run();
```

The architectural rule becomes:

$$
Implementation\ Selection \in app
$$

while:

$$
Business\ Logic \notin app
$$

This is a very important Rust pattern.

`main.rs` becomes boring.

That's good.

---

# Level 6 — enforce the architecture

Now deliberately try to violate your rules.

For example:

```text
domain → service
```

should be forbidden.

Then introduce tooling/tests to make those architectural constraints enforceable.

Your progression becomes:

$$
\text{convention}
\rightarrow
\text{visibility}
\rightarrow
\text{Cargo dependency}
\rightarrow
\text{tests}
\rightarrow
\text{automated architectural check}
$$

This is where "modularity" stops being a diagram and becomes an **engineering constraint**.

---

# Level 7 — make an abstraction genuinely useful

Don't immediately create ten traits.

Instead introduce one replaceable resource:

```text
Clock
```

Define:

```rust
pub trait Clock {
    fn now(&self) -> u64;
}
```

Then make:

```text
SystemClock
FakeClock
```

implement it.

Now your service can be tested without the real clock.

You're learning:

$$
Concrete\ Dependency
\rightarrow
Capability
\rightarrow
Multiple\ Implementations
$$

This is much closer to the capability-oriented design you've been thinking about.

---

# Level 8 — finally introduce state

Only after the previous levels, introduce:

```text
State
Service
Repository
App
```

For example:

```text
app
 ↓
service
 ↓
repository trait
 ↙        ↘
memory   sqlite
```

Now you can practice:

* ownership
* borrowing
* `&mut`
* state boundaries
* traits
* dependency injection
* composition
* testing

without simultaneously learning Cargo.

---

# Your progression

I'd actually make the entire curriculum this:

$$
\boxed{
\begin{array}{c}
1.\ app \rightarrow greeter\
2.\ app \rightarrow formatter \rightarrow greeter\
3.\ app \rightarrow service \rightarrow domain\
4.\ service \rightarrow port \leftarrow implementation\
5.\ app = composition\ root\
6.\ enforce\ dependency\ graph\
7.\ capability + multiple\ implementations\
8.\ state + repository
\end{array}}
$$

And keep each project **tiny**—ideally 20–100 lines.

The rule I'd use for yourself is:

> **Never introduce two new architectural concepts in the same exercise.**

You just learned **crate composition**.

So your next exercise should specifically teach **dependency transitivity + hiding**, not async, traits, state, databases, or an elaborate kernel.

### Your immediate next task

Create:

```text
app → formatter → greeter
```

with the constraint:

$$
app \not\rightarrow greeter
$$

Then use `cargo tree` to prove that your dependency graph is what you intended.

Once you've done that, the next step is **crate purity (`domain` with zero dependencies)**.
