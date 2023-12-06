# Hirola

[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Browser Tests](https://github.com/geofmureithi/hirola/actions/workflows/browser.yml/badge.svg)](https://github.com/geofmureithi/hirola/actions/workflows/browser.yml)
[![Unit Tests](https://github.com/geofmureithi/hirola/actions/workflows/unit.yml/badge.svg)](https://github.com/geofmureithi/hirola/actions/workflows/unit.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Hirola** is a declarative frontend framework that is focused on simplicity and reactivity.

## Goals

1. KISS: A simple and declarative way to build frontend UIs in rust.
2. Make it easy to read, extend and share code.
3. Frp signals allowing fine-grained reactivity.
4. Familiarity: Uses rsx which is very similar to jsx.

## Example

We are going to create a simple counter program.

```bash
cargo new counter
```

With a new project, we need to create an index file which is the entry point and required by trunk

```bash
cd counter
```

Create an `index.html` in the root of counter. Add the contents below

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
    <body></body>
  </head>
</html>
```

Lets add some code to `src/main.rs`

```rust,no_run
use hirola::prelude::*;
use hirola::dom::Dom;

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
    let root = hirola::dom::render(counter()).unwrap();
    std::mem::forget(root);
}
```

Now lets run our project

```bash
trunk serve
```

## Ecosystem

Check out [Hirola Docs](https://hirola-docs.vercel.app/basics/getting-started) written with Hirola itself!

Here are some extensions for hirola:

1. [Form](https://hirola-docs.vercel.app/plugins/form)
2. [Router](https://hirola-docs.vercel.app/plugins/router)
3. [State](https://hirola-docs.vercel.app/plugins/state)
4. [Markdown](https://hirola-docs.vercel.app/plugins/mdx)

### Milestones

| Status | Goal                               | Labels    |
| :----: | :--------------------------------- | --------- |
|   ✔    | Basic templating with rust and rsx | `ready`   |
|   ✔    | Extend functionality with mixins   | `ready`   |
|   ✔    | Components                         | `ready`   |
|   ✔    | SSR                                | `ready`   |
|   ✔    | Signals                            | `ready`   |
|   🚧   | Form management                    | `started` |
|   ⏳   | Markdown templating                | `pending` |
|   🚧   | Styling                            | `started` |
|   ⏳   | SSG                                | `pending` |
