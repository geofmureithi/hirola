# Hirola

[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Build Status](https://travis-ci.org/geofmureithi/hirola.svg?branch=master)](https://travis-ci.org/geofmureithi/hirola)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Hirola** is an opinionated web framework for that is focused on simplicity and predictability.

Here is a simple example:

```rust
use hirola::prelude::*;

fn counter(_: &HirolaApp) -> Dom {
    let state = Signal::new(99);
    let decerement = state.event_callback(|count, _| *count - 1);
    let incerement = state.event_callback(|count, _| *count + 1);

    html! {
        <div>
            <button on:click={decerement}>"-"</button>
            <input value={state.get()} disabled/>
            <button on:click={incerement}>"+"</button>
        </div>
    }
}

fn main() {
    let mut app = HirolaApp::new();
    app.mount("body", counter);
}

```

### Milestones

| Status | Goal                                                                      | Labels        |
| :----: | :------------------------------------------------------------------------ | ------------- |
|   ✔    | Write code that is declarative and easy to follow                         | `ready`       |
|   ❌   | [Standardize Components](https://github.com/geofmureithi/hirola/issues/1) | `help wanted` |
|   🚀   | SSR First Approach                                                        | `help wanted` |
|   🚀   | Hydration                                                                 | `help wanted` |
|   🚀   | Serverside integrations                                                   | `help wanted` |

### Inspiration

- Sycamore/Maple
- Alpine.js
- React.js
- Yew

#### Demo examples

> This API will certainly change.

Go to `examples` and use trunk

```
$  trunk serve
```

#### Prerequisite:

You need need to have `rust`, `cargo` and `trunk` installed.

License: MIT
