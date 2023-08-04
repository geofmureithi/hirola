---
title: Reactivity with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: We are going to learn how to handle reactivity using hirola
draft: false
---

# Reactivity

Hirola offers reactivity via futures-signals using mainly `Mutable`, `MutableVec` and `MutableBtreeMap`. Once a signal is updated, these changes are propagated to the dom.

> Hirola uses frp-signals reactivity under the hood to provide these functions.[→ Read more about frp reactivity](https://crates.io/crates/futures-signals)

## Reactive Signal

```rust
use hirola_core::prelude::*;
let count = Mutable::new(0i32);
assert_eq!(count.get(), 0);

count.set(1);
assert_eq!(count.get(), 1);
```

## Subscribing

Subscribing is done via polling a future:

```rust
use hirola_core::prelude::*;
let state = Mutable::new(0);
assert_eq!(state.get(), 0);
state.to_signal().for_each(|value| {
    // do something with new value
});
/// later
state.set(1);
```
