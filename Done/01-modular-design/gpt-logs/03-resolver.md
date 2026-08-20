Your intent is **Cargo workspace semantics**: understand what `resolver = "3"` changes in the dependency-resolution function, rather than treating it as magic configuration.

Given:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

there are actually **two independent declarations**.

---

## 1. `members`

```toml
members = ["app", "greeter"]
```

This defines the workspace's package set.

Let:

$$
P = {app,\ greeter}
$$

Then:

$$
Workspace(P)
$$

means Cargo manages these packages as members of one workspace.

Importantly, this **doesn't establish a dependency**:

$$
app \not\rightarrow greeter
$$

You still need this in `app/Cargo.toml`:

```toml
[dependencies]
greeter = { path = "../greeter" }
```

which establishes:

$$
app \rightarrow greeter
$$

So:

```text
workspace membership
```

and:

```text
crate dependency
```

are two different relations.

---

# 2. What is `resolver`?

Cargo has to solve a **dependency-version selection problem**.

Suppose:

```text
app
 ├── A
 │    └── foo ^1.0
 │
 └── B
      └── foo ^2.0
```

Cargo has to determine which versions of `foo` are included in the build.

Conceptually, define:

$$
D = \text{all dependency requirements}
$$

and:

$$
V = \text{available package versions}
$$

Cargo computes something like:

$$
R : D \rightarrow V^*
$$

where $R$ is the **dependency resolver** and $V^*$ is a valid collection of selected package versions satisfying the dependency constraints.

The `resolver` setting selects **the rules Cargo uses for this function**.

---

# 3. Why does resolver version matter?

Cargo has had several dependency-resolution algorithms/semantics.

The important versions are:

```text
resolver = "1"
resolver = "2"
resolver = "3"
```

For modern Rust projects, **`resolver = "3"` is the current resolver behavior and is the appropriate choice for a new project using the current edition**.

The differences matter primarily around **feature unification and dependency kinds**.

---

# 4. The key idea: features

Suppose:

```toml
foo = { version = "1", features = ["json"] }
```

and somewhere else:

```toml
foo = { version = "1", features = ["networking"] }
```

Cargo generally doesn't build two independent copies merely because different features are requested.

Conceptually:

$$
Features(foo) =
Features_1(foo)
\cup
Features_2(foo)
$$

so:

$$
Features(foo)={json, networking}
$$

This is called **feature unification**.

The resolver determines the rules governing when these features are unified.

---

# 5. Why resolver 2/3 are better

The older resolver had situations where features could be unified more broadly than desired across dependency contexts.

For example, distinguish:

$$
DependencyKind =
{Normal,\ Dev,\ Build}
$$

A development dependency:

```toml
[dev-dependencies]
foo = { ... }
```

is conceptually different from a normal runtime dependency:

```toml
[dependencies]
foo = { ... }
```

Modern resolver semantics make these contexts more independent in cases where they don't need to influence one another.

So instead of treating every feature request as belonging to one giant global set:

$$
F(foo)=\bigcup_i F_i(foo)
$$

the resolver considers the **dependency context**.

You can think of the resolution input as:

$$
D = (Package,\ Dependency,\ Context,\ Features,\ VersionReq)
$$

rather than simply:

$$
D=(Package,\ Features,\ VersionReq)
$$

That distinction becomes important in larger workspaces.

---

# 6. Why `"3"` specifically?

For your project:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

you are telling Cargo:

> "Treat `app` and `greeter` as one workspace, and use dependency resolver version 3 for resolving their dependencies."

It **does not mean**:

> "There are three dependency levels."

It has nothing to do with:

```text
app → greeter
```

and nothing to do with how many crates you have.

It's simply the identifier of Cargo's resolver semantics.

---

# 7. Relation to Rust editions

The modern Rust ecosystem ties resolver defaults to editions.

In particular, **edition 2024 implies resolver 3 when Cargo can infer the resolver from the workspace configuration**, but explicitly writing:

```toml
resolver = "3"
```

at the workspace root makes the intended behavior obvious and avoids ambiguity.

That's why you'll commonly see:

```toml
[workspace]
members = ["crates/*"]
resolver = "3"
```

in a modern workspace.

---

# 8. Formal picture of your workspace

Your configuration:

```toml
[workspace]
members = ["app", "greeter"]
resolver = "3"
```

can be modeled as:

$$
W=(P,R)
$$

where:

$$
P={app,greeter}
$$

and:

$$
R=Resolver_3
$$

Cargo then processes each package's manifest to construct dependency requirements:

$$
Req(W)=
{req_1,\ldots,req_n}
$$

and computes:

$$
Resolver_3(Req(W))
\rightarrow
Lockfile/BuildGraph
$$

Then Rust compilation operates over the resulting graph.

So the pipeline is roughly:

$$
\boxed{
Cargo.toml
\rightarrow
Workspace
\rightarrow
Dependency\ Requirements
\rightarrow
Resolver_3
\rightarrow
Resolved\ Graph
\rightarrow
rustc
\rightarrow
Artifacts
}
$$

That distinction is worth internalizing:

**`members` defines the set being managed.**

**`resolver` defines the rules used to resolve dependencies within that set.**

**`[dependencies]` defines the actual dependency edges.**

For your current tiny project, `resolver = "3"` won't visibly change anything because `app` and `greeter` have essentially no interesting dependency feature interactions yet. You're adding it now because you're establishing the **modern production workspace baseline**, not because the toy project needs its sophisticated resolution behavior.
