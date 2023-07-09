# Hirola

[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Browser Tests](https://github.com/geofmureithi/hirola/actions/workflows/browser.yml/badge.svg)](https://github.com/geofmureithi/hirola/actions/workflows/browser.yml)
[![Unit Tests](https://github.com/geofmureithi/hirola/actions/workflows/unit.yml/badge.svg)](https://github.com/geofmureithi/hirola/actions/workflows/unit.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Hirola** is an un-opinionated webf ramework that is focused on simplicity and predictability.

## Goals

1. Keep it simple. A simple and declarative way to build web UIs in rust with a small learning curve.
2. Make it easy to read, extend and share code. Mixins and components are kept simple and macro-free.
3. No context, you can choose passing props down.
4. Familiality. Uses rsx which is very similar to jsx.

## Example

We are going to create a simple counter program.

```
cargo new counter
```

With a new project, we need to create an index file which is the entry point and required by trunk

```
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

```rust
use hirola::prelude::*;

fn counter() -> Dom {
    let count = Mutable::new(0);
    let increment = count.update(|c| c + 1);
    let decrement = count.update(|c| c - 1);
    html! {
        <div>
            <button on:click=decrement>"-"</button>
            <span>{count}</span>
            <button on:click=increment>"+"</button>
        </div>
    }
}
fn main() {
    hirola::render(counter);
}
```

Now lets run our project

```
trunk serve
```

You should be able to get counter running: [Live Example](https://hirola-docs.vercel.app/basics/getting-started)

## Ecosystem

Check out [Hirola Docs](https://hirola-docs.vercel.app/basics/getting-started) written with Hirola itself!

Here are some extensions for hirola:

1. [Form](https://hirola-docs.vercel.app/plugins/form)
2. [Router](https://hirola-docs.vercel.app/plugins/router)
3. [State](https://hirola-docs.vercel.app/plugins/state)

### Milestones

| Status | Goal                                                                      | Labels  |
| :----: | :------------------------------------------------------------------------ | ------- |
|   ✔    | Write code that is declarative and easy to follow                         | `ready` |
|   ✔    | Allow extensibility via mixins                                            | `ready` |
|   ✔    | [Standardize Components](https://github.com/geofmureithi/hirola/issues/1) | `ready` |
|   ✔    | SSR                                                                       | `ready` |

### Inspiration

- Sycamore
- Alpine.js
- React.js
- Yew
- Choojs

#### Demo examples

> This API will certainly change.

Go to `examples` and use trunk

```
$  trunk serve
```

#### Prerequisite:

You need need to have `rust`, `cargo` and `trunk` installed.

License: MIT
