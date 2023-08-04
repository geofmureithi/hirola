---
title: Hirola - A KISS Rust frontend framework
date: '2023-01-21'
tags: ['rust', 'hirola', 'basics', 'starter']
summary: Hirola is a frontend framework for Rust that is focused on simplicity and predictability
draft: false
---

# What is Hirola?

**Hirola** is a frontend framework for Rust that is focused on simplicity and predictability.

## Goals

- Keep it simple.
- Make it easy to read, extend and share code. Mixins and components are kept simple and \*macro-free.
- Basic state management that is easily extensible
- Familiarity. Uses rsx which is very similar to jsx.

## Example

```rust
use std::fmt::Display;
use hirola::prelude::*;
use hirola::signal::Mutable;

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    html! {
        <>
            <button on:click=decrement>"-"</button>
            <span>{count}</span>
            <button on:click=increment>"+"</button>
        </>
    }
}

fn main() {
    let root = render(counter()).unwrap();
    // We prevent the root from being dropped
    std::mem::forget(root);
}
```

## Features

- **`serde`**— Enables serialization of state
- **`ssr`**— Enables server side rendering 🚧
- **`router`**— Enables Isomorphic Routing
- **`form`**— Enables form mixins and utilities 🚧
