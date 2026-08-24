**Intent: Rust type inference + iterator algebra.** The confusing part is that there are three separate pieces here:

```rust
let pairs = (0..4)
    .map(|_| (rand::random(), rand::random()))
    .collect::<Vec<_>>();
```

## 1. What does `|_|` mean?

`map` has the conceptual type:

$$
\operatorname{map} : (A\to B)\to \operatorname{Iterator}(A)\to\operatorname{Iterator}(B)
$$

Your iterator is:

```rust
0..4
```

so its elements are:

$$
0,1,2,3
$$

Therefore `map` gives your closure **one element at a time**.

```rust
.map(|x| ...)
```

would mean:

> take each element and bind it to `x`.

But you don't need the element.

So:

```rust
.map(|_| ...)
```

means:

> take each element, but deliberately discard it.

Thus:

```rust
.map(|_| (rand::random(), rand::random()))
```

means:

$$
0\mapsto(n_0,m_0)
$$

$$
1\mapsto(n_1,m_1)
$$

$$
2\mapsto(n_2,m_2)
$$

$$
3\mapsto(n_3,m_3)
$$

The input is only being used to determine **how many times** the closure runs.

---

## 2. Why `Vec<_>`?

Before `collect`, you have an iterator:

$$
I((u64,u64))
$$

`collect` turns that iterator into some collection:

$$
I(A)\to C(A)
$$

But Rust needs to know **which collection**.

You tell it:

```rust
.collect::<Vec<_>>()
```

meaning:

> collect these elements into a `Vec`, and infer the element type.

The `_` means **type inference placeholder**.

So Rust effectively figures out:

```rust
Vec<(u64, u64)>
```

assuming `rand::random()` is inferred as `u64` from your later usage.

You could write the fully explicit version:

```rust
.collect::<Vec<(u64, u64)>>()
```

but that's unnecessarily verbose here.

---

## 3. Why the `::<...>`?

This:

```rust
collect::<Vec<_>>()
```

is **turbofish syntax**.

The `collect` method is generic:

```rust
collect::<C>()
```

where `C` is the collection type you want.

So:

```rust
collect::<Vec<_>>()
```

says:

$$
C=\operatorname{Vec}
$$

The `()` afterward is simply the **function/method call**.

Compare:

```rust
foo::<T>()
```

There are two different things:

```text
::<T>   → specify generic type argument
()      → call the function
```

For example:

```rust
collect::<Vec<_>>()
       ^^^^^^^^^^
       type argument

                ^^
                actually invoke collect
```

---

## 4. Why can't Rust just infer `Vec`?

Sometimes it can.

For example, if you explicitly annotate the variable:

```rust
let pairs: Vec<(u64, u64)> = (0..4)
    .map(|_| (rand::random(), rand::random()))
    .collect();
```

Now `.collect()` knows its target is `Vec<(u64, u64)>`, so you don't need turbofish.

These are essentially equivalent:

```rust
let pairs = iterator.collect::<Vec<_>>();
```

and:

```rust
let pairs: Vec<_> = iterator.collect();
```

The information is just being supplied at different locations.

---

### The important mental model

Your whole expression is:

$$
{0,1,2,3}
\xrightarrow{\text{map}}
{(n_0,m_0),(n_1,m_1),(n_2,m_2),(n_3,m_3)}
\xrightarrow{\text{collect}}
\operatorname{Vec}((u64,u64))
$$

And `|_|` means:

$$
f:\mathbb N\to\mathbb N^2
$$

where the function **ignores its input**:

$$
f(x)=(\operatorname{random}(),\operatorname{random}())
$$

So the `0..4` is effectively supplying **four evaluation opportunities**, not meaningful data.
