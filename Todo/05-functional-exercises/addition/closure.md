**Intent: set-theoretic formalization of Rust closures.**

The key distinction is:

> In pure set theory, a **closure** is not a special kind of function. A closure is best modeled as a **function together with an environment** that supplies the free variables used by its body.

### 1. Ordinary mathematical function

A function is a set of ordered pairs satisfying functionality:

$$
f\subseteq A\times B
$$

such that:

$$
\forall x\in A,;\exists!y\in B:(x,y)\in f
$$

We write:

$$
f:A\to B
$$

For:

```rust
fn add_one(x: u64) -> u64 {
    x + 1
}
```

you can model it simply as:

$$
f:U\to U
$$

where:

$$
U={0,\ldots,2^{64}-1}
$$

---

## 2. Where closure differs

Consider:

```rust
let n = 10;

let f = |x| x + n;
```

The expression `x + n` contains:

* `x`: **bound variable** — supplied when calling `f`
* `n`: **free variable** — supplied by the surrounding environment

So mathematically, the body is initially something like:

$$
e(x,n)=x+n
$$

But the closure captures the particular environment:

$$
\rho={n\mapsto10}
$$

Therefore the closure can be modeled as:

$$
(\rho,e)
$$

That is the fundamental idea.

---

## 3. Closure evaluation

Define an evaluation operation:

$$
\operatorname{eval}(e,\rho)
$$

For your closure:

$$
\operatorname{eval}(x+n,{n\mapsto10})
$$

produces a function:

$$
x\mapsto x+10
$$

So:

$$
\boxed{
(\rho,e)\mapsto f
}
$$

where:

$$
f:U\to U
$$

In other words:

$$
\boxed{
\text{closure}=(\text{environment},\text{function body})
}
$$

and **applying/evaluating the closure under its environment produces an ordinary function**.

---

## 4. Your `fold` closure

You have:

```rust
(0..m).fold(n, |value, _| value + 1)
```

Here the closure:

```rust
|value, _| value + 1
```

has **no free variables**.

Both `value` and `_` are parameters.

So its environment is empty:

$$
\rho=\varnothing
$$

and its body is:

$$
e(v,i)=v+1
$$

Therefore:

$$
C=(\varnothing,e)
$$

and its effective function is:

$$
C:(U\times U)\to U
$$

with:

$$
C(v,i)=v+1
$$

This is why your closure is particularly simple: **it is a closure syntactically, but it has an empty environment**.

---

## 5. Compare with your earlier `f`

You wrote:

```rust
fn f(value: u64) -> u64 {
    value + 1
}
```

This has no environment:

$$
f:U\to U
$$

Your closure:

```rust
|value, _| value + 1
```

has:

$$
C:U\times U\to U
$$

but:

$$
\rho_C=\varnothing
$$

So the conceptual distinction is:

$$
\begin{aligned}
\text{ordinary function}
&\approx e\
\text{closure}
&\approx(\rho,e)
\end{aligned}
$$

with closure application:

$$
(\rho,e)(x)=\operatorname{eval}(e,\rho[x\mapsto x])
$$

where:

$$
\rho[x\mapsto x]
$$

extends the environment with the function argument.

---

## 6. Why this matters in Rust

Now consider:

```rust
let n = 10;

let f = |x| x + n;
```

The closure must somehow retain access to `n`.

Conceptually Rust creates something resembling:

```rust
struct Closure {
    n: u64,
}
```

and behavior resembling:

```rust
impl Closure {
    fn call(&self, x: u64) -> u64 {
        x + self.n
    }
}
```

**Don't take this as literal compiler representation**—it's the useful semantic model.

Set-theoretically:

$$
\rho={n\mapsto10}
$$

and:

$$
e(x,n)=x+n
$$

so:

$$
(\rho,e)(x)=x+10
$$

This is also why Rust's closure traits (`Fn`, `FnMut`, `FnOnce`) are about more than the mathematical function $A\to B$: they describe **how the closure's captured environment can be accessed**.

### The most useful formal picture

$$
\boxed{
\begin{array}{ccc}
\text{Closure} & = & (\rho,e)\
&&\downarrow\operatorname{apply}\
&&f:A\to B
\end{array}
}
$$

where:

* $\rho$ = captured environment
* $e$ = function body with free variables
* `apply` supplies arguments and evaluates the body under $\rho$

So when you see a Rust closure, first ask:

**"What are its free variables?"**

Those are precisely the things that determine its captured environment.
