**Intent: operational semantics / iterator evaluation order.**

Yes: read the iterator chain **left-to-right as composition**, but distinguish **construction** from **consumption**.

### Hint for your `step` + iteration version

Think:

$$
s_0=(n,0)
$$

and

$$
s_{k+1}=\operatorname{step}(s_k)
$$

You want to apply `step` **exactly `m` times**, then extract the first component.

Rust has an iterator abstraction that is perfect for expressing:

> "start with one state, repeatedly produce the next state."

Hint toward:

```rust
std::iter::successors(...)
```

or, if you want to practice more explicitly, think about:

```rust
(0..m).fold(...)
```

The latter is probably the better exercise if your goal is understanding the algebra.

---

### Your pipeline

For:

```rust
pairs
    .iter()
    .map(...)
    .zip(...)
    .all(...)
```

conceptually:

$$
\texttt{pairs}
\xrightarrow{\texttt{iter}}
I(\texttt{pairs})
\xrightarrow{\texttt{map}}
I(A)
\xrightarrow{\texttt{zip}}
I(A\times B)
\xrightarrow{\texttt{all}}
{\texttt{true},\texttt{false}}
$$

But **it is lazy**.

So this:

```rust
let result = pairs
    .iter()
    .map(...)
    .zip(...)
    .all(...);
```

does **not** mean Rust eagerly executes:

1. all of `iter`
2. all of `map`
3. all of `zip`
4. all of `all`

Instead, `all` is the **consumer**. It pulls an element from the pipeline, and that demand propagates backwards.

For one element, roughly:

```text
all asks for next
    ↓
zip produces next pair
    ↓
left map produces next
    ↓
left iter produces next pair

and simultaneously

zip asks right map for next
    ↓
right iter produces next pair
```

Then `all` tests the resulting pair.

So the actual evaluation is more like:

$$
\texttt{all}
\leftarrow
\texttt{zip}
\leftarrow
\begin{cases}
\texttt{map}\leftarrow\texttt{iter}\
\texttt{map}\leftarrow\texttt{iter}
\end{cases}
$$

**demand flows from the consumer backward through the iterator adapters**, while the values themselves flow forward.

And because `all` short-circuits, it may only evaluate part of `pairs`.

One subtle point: `.map()` itself doesn't run the closure when you call `.map()`. The closure runs when the downstream consumer requests an item.
