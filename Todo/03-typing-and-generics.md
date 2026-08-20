Your intent is **type-system mastery through adversarial exercises**: not learning syntax, but testing whether you can predict what a type checker permits, rejects, or infers—and why.

I'd structure this as a progression from **parametric polymorphism → variance/subtyping → associated types/type families → higher-order generics → typestate → trait bounds**.

## Level 1 — Parametric polymorphism

### 1. Identity and information preservation

**Python**

Define:

```python
def identity(x):
    ...
```

What is the most precise type you can give it using `TypeVar`?

Then answer:

1. What does `identity(3)` have as its inferred type?
2. What does `identity("x")` have?
3. Can the implementation inspect `x` and still satisfy the same generic contract?
4. Why is `def identity(x: object) -> object` weaker?

**Rust**

Implement:

```rust
fn identity<T>(x: T) -> T
```

Then explain algebraically why this is stronger than:

```rust
fn identity(x: Box<dyn Any>) -> Box<dyn Any>
```

---

### 2. Generic container transformation

Define a function:

```text
map : (A → B) → List[A] → List[B]
```

Implement it in both Python and Rust.

Then modify the problem:

```text
filter : (A → Bool) → List[A] → List[A]
```

Ask yourself:

> Why does `map` need two type parameters while `filter` only needs one?

Formally, identify the type-level morphism being represented.

---

### 3. The generic composition trap

What should this function's type be?

```python
def compose(f, g):
    return lambda x: f(g(x))
```

You want:

```text
g : A → B
f : B → C

compose(f, g) : A → C
```

Now try to express this using Python's `TypeVar`.

Then implement the equivalent in Rust.

**Challenge:** explain why Python's type system can express the *intent* less precisely than Rust's compiler can enforce it.

---

# Level 2 — Variance

This is where your understanding should get uncomfortable.

### 4. Is this assignment valid?

Suppose:

```python
class Animal: ...
class Dog(Animal): ...
```

Determine whether each is sound:

```python
dogs: list[Dog] = ...
animals: list[Animal] = dogs
```

Now replace `list` with:

```python
Callable[[Dog], None]
Callable[[Animal], None]
```

Determine which direction of substitution is valid.

Don't memorize "covariant/contravariant."

Instead derive it from:

> **What operations does the consumer gain permission to perform?**

Then formalize the result as a relation between sets of valid programs.

---

### 5. Rust variance challenge

Consider:

```rust
fn use_animal(f: impl Fn(Animal)) {
    ...
}
```

and:

```rust
fn use_dog(f: impl Fn(Dog)) {
    ...
}
```

Construct examples showing why function arguments behave contravariantly.

Then compare:

```rust
Vec<Dog>
Vec<Animal>
```

Why doesn't Rust allow the same substitution?

The key question:

> What operation would become unsound if `Vec<Dog>` were treated as `Vec<Animal>`?

---

# Level 3 — Trait bounds / protocols

### 6. "Can I call this method?"

Given:

```rust
trait Speak {
    fn speak(&self);
}

fn talk<T>(x: T) {
    x.speak();
}
```

Why doesn't this compile?

Fix it with the weakest appropriate constraint.

Then compare:

```rust
fn talk<T: Speak>(x: T)
```

with:

```rust
fn talk(x: impl Speak)
```

and:

```rust
fn talk(x: &dyn Speak)
```

Explain the semantic difference between all three.

---

### 7. Conditional generic behavior

Define:

```rust
fn process<T>(x: T)
```

such that it only accepts `T` when:

```text
T : Clone + Debug + Send
```

Then ask:

> Is this constraint part of the function's implementation, or part of its mathematical domain?

Formalize the function as:

$$
f : D \to R
$$

and describe how the trait bounds determine the subset:

$$
D \subseteq \mathcal{U}
$$

of values for which `f` is defined.

---

# Level 4 — Associated types

### 8. Generic parameter vs associated type

Create:

```rust
trait IteratorLike {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Now imagine instead:

```rust
trait IteratorLike<Item> {
    fn next(&mut self) -> Option<Item>;
}
```

Explain the difference.

Then answer:

> Why can a type implement the first trait for exactly one `Item`, while the second potentially permits multiple implementations?

This is an important distinction between:

$$
F : A \to B
$$

and a relation:

$$
R \subseteq A \times B
$$

The associated type behaves more like a **type-level function**.

---

### 9. Associated-type equality

Suppose:

```rust
trait Container {
    type Item;
}

fn consume<C>(x: C)
where
    C: Container<Item = String>,
{
    ...
}
```

What exactly does:

```rust
C: Container<Item = String>
```

assert?

Then generalize it to:

```text
C → C::Item
```

and reason about equality constraints between type-level morphisms.

---

# Level 5 — Type-level programming

### 10. Generic transformation

Create:

```rust
trait Transform<T> {
    type Output;

    fn transform(x: T) -> Self::Output;
}
```

Implement:

```text
String → usize
usize → String
Dog → Animal
```

Then ask:

> Is `Transform` fundamentally a function, a relation, or an object containing a function?

Formalize the distinction.

---

### 11. Type-level composition

Suppose:

```text
F : A → B
G : B → C
```

You want to represent:

$$
G \circ F : A \to C
$$

using Rust traits.

Try designing a generic:

```rust
Compose<F, G>
```

without looking up existing patterns.

The challenge is figuring out where the intermediate type `B` comes from.

---

# Level 6 — Typestate

### 12. Make illegal states unrepresentable

Design:

```text
Connection<Disconnected>
Connection<Connected>
```

with operations:

```text
connect()
send()
disconnect()
```

Requirements:

```text
Disconnected → Connected
Connected → Disconnected
Connected → Connected
```

but:

```text
Disconnected.send()
```

must be impossible to compile.

Do this in Rust.

Then answer:

> What has moved from runtime state into the type system?

Formally, you're replacing:

$$
State \times Operation \to State
$$

with something closer to a family:

$$
Operation : S_i \to S_j
$$

where the compiler tracks `S_i` and `S_j`.

---

# Level 7 — Python Protocols vs Rust traits

### 13. Structural vs nominal typing

Define the same abstraction in Python:

```python
class Serializable(Protocol):
    def serialize(self) -> bytes: ...
```

and Rust:

```rust
trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}
```

Now create a type that already has the correct method but was never explicitly declared to implement the abstraction.

Ask:

1. Does Python accept it?
2. Does Rust accept it?
3. What kind of typing is each system using?
4. What information does the compiler/type checker need?
5. What does "implements an interface" mean set-theoretically?

This should lead you to distinguish:

$$
\text{Structural compatibility}
$$

from

$$
\text{Nominal membership}
$$

---

# Level 8 — Higher-order generics

### 14. Generic callback container

Design:

```rust
struct Processor<F> {
    f: F,
}
```

where:

```text
F : A → B
```

and implement:

```rust
process(x: A) -> B
```

Then make `Processor` generic over the input and output types.

Now answer:

> What is actually being parameterized—data, behavior, or both?

Then compare this with:

```rust
Box<dyn Fn(A) -> B>
```

What did you give up by erasing the concrete type?

---

# Level 9 — The hard ones

### 15. Generic state machine

Design:

```text
Machine<S>
```

with states:

```text
Start
Running
Stopped
Failed
```

and transitions:

```text
Start → Running
Running → Stopped
Running → Failed
Failed → Running
```

Make the following impossible statically:

```text
Stopped → Running
Start → Stopped
Stopped → Failed
```

Do **not** use runtime enums for the state.

---

### 16. Generic ownership API

Design a Rust abstraction:

```text
Resource<R>
```

where `R` represents the resource authority.

You want:

```text
Resource<Readable>
Resource<Writable>
Resource<ReadWrite>
```

with:

```text
read : Readable → Data
write : Writable × Data → Writable
```

Then determine whether:

```text
ReadWrite <: Readable
ReadWrite <: Writable
```

should exist at the type level.

This exercise combines:

* generics
* trait bounds
* subtyping
* capability modeling
* ownership
* variance

which is very close to the kind of reasoning you've been doing with your runtime/kernel abstractions.

---

# Level 10 — Final challenge: design a typed runtime

Build a miniature runtime with:

```text
Application
Session
Conversation
Agent
```

Each should have a type parameter representing its lifecycle state.

For example:

```text
Application<Bootstrapping>
Application<Running>
Application<ShuttingDown>
```

and:

```text
Session<Unauthenticated>
Session<Authenticated>
```

Require that:

```text
Application<Running>
    → Session<Authenticated>
    → Conversation<Active>
    → Agent<Running>
```

while preventing invalid transitions statically.

Then introduce **capability types**:

```text
StorageCapability
ModelCapability
EventCapability
CommandCapability
```

and make the agent generic over the capabilities it possesses.

The final question is:

> **What belongs in the value/state space, and what belongs in the type space?**

That distinction is the real objective of these exercises.

### Suggested progression

I would do them in this order:

$$
\boxed{
\text{Parametricity}
\rightarrow
\text{Variance}
\rightarrow
\text{Bounds}
\rightarrow
\text{Associated Types}
\rightarrow
\text{Type-Level Functions}
\rightarrow
\text{Typestate}
\rightarrow
\text{Capabilities}
\rightarrow
\text{Typed Runtime}
}
$$

For each exercise, don't just make it compile. Write down four things:

1. **Domain** — what values/types are admitted?
2. **Codomain** — what can the operation produce?
3. **Constraints** — what relationships must hold between the types?
4. **Information loss** — what does moving from concrete generics to `object`/`Any`/`dyn Trait` erase?

That will push you toward understanding generics as **mathematical structure**, rather than as syntax for reusable functions.
