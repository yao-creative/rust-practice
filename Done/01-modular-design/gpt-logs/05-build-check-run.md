Your intent is **Cargo command semantics**: distinguish *checking*, *building*, and *executing* a selected workspace package.

When you ran:

```bash
cargo run -p app
```

Cargo is doing **two phases**:

$$
Source
\xrightarrow{\text{compile}}
Executable
\xrightarrow{\text{execute}}
Process
$$

That's why `run` both builds and runs.

### The three commands

**`cargo check`**

```bash
cargo check -p app
```

Checks whether the code compiles, but doesn't produce the final executable.

$$
Source \rightarrow \text{type/borrow/compile validation}
$$

It's fast and is what you should use while developing.

---

**`cargo build`**

```bash
cargo build -p app
```

Actually compiles the executable and puts the artifact under:

```text
target/debug/app
```

Conceptually:

$$
Source_{app}
+
Dependencies
\xrightarrow{rustc}
Executable_{app}
$$

It **doesn't start the program**.

---

**`cargo run`**

```bash
cargo run -p app
```

is essentially:

```bash
cargo build -p app
./target/debug/app
```

Cargo first makes sure the executable is up to date, then launches it.

So:

$$
cargo\ run
==========

cargo\ build
+
execute
$$

---

## What does `-p app` mean?

`-p` means **package selection**.

Because your workspace contains:

```text
mini/
├── app
└── greeter
```

you can say:

```bash
cargo build -p app
```

or:

```bash
cargo check -p greeter
```

or:

```bash
cargo run -p app
```

The workspace is the universe:

$$
P={app,greeter}
$$

and:

```bash
-p app
```

selects:

$$
{app}\subseteq P
$$

Cargo then also builds whatever `app` depends on.

Since you have:

```text
app → greeter
```

running:

```bash
cargo run -p app
```

causes Cargo to resolve/build:

$$
greeter \rightarrow app \rightarrow execute(app)
$$

You **don't run `greeter`**, because it's a library crate. It is a dependency artifact consumed by `app`.

### Practical development loop

For your current exercise, use:

```bash
cargo check
```

most of the time.

When you want to actually see the program:

```bash
cargo run -p app
```

When you specifically want to inspect the produced binary:

```bash
cargo build -p app
```

And when you're learning the dependency graph:

```bash
cargo tree
```

That gives you a very clean mental separation:

$$
\boxed{
check = validate
\qquad
build = produce
\qquad
run = produce + execute
}
$$
