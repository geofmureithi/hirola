---
title: Getting started with hirola.
date: '2023-01-21'
tags: ['rust', 'hirola', 'basics', 'starter']
summary: We are going to create a simple counter program using hirola
draft: false
---

# Prerequisites

Before getting started with `hirola` we are going to assume that you have the following tools installed:

- rust
- cargo
- hirola-kit

# Getting Started

We are going to create a simple counter program.

`cargo new counter`

With a new project, we need to create an index file which is the entry point and required by trunk

`cd counter`

Create an **index.html** in the root of counter. Add the contents below

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
  </head>
</html>
```

Lets add some code to **src/main.rs**

```rust,no_run
use hirola::prelude::*;

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

Now lets run our project

`hirola-kit serve`

You should be able to get counter running.
