# Hirola

[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Build Status](https://travis-ci.org/geofmureithi/hirola.svg?branch=master)](https://travis-ci.org/geofmureithi/hirola)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Hirola** is an opinionated web framework for that is focused on simplicity and predictability.

Here is a simple example:

```rust
use hirola::prelude::*;

fn Home() -> TemplateResult<DomNode> {
    let state = Signal::new(99);
    let decerement = state.reduce_callback(|count, _| *count - 1);
    let incerement = state.reduce_callback(|count, _| *count + 1);

    html! {
        <div class="flex flex-row h-10">
            <button on:click={decerement}>"-"</button>
            <input value={state.get()} disabled/>
            <button on:click={incerement}>"+"</button>
        </div>
    }
}

fn main() {
    let mut app = HirolaApp::new();
    app.router("/", Home)
    app.mount("body");
}

```

### Goals

- [x] Write code that is declarative and easy to follow.
- [x] Follow Alpine-ish kind of Reactive and declarative style.
- [ ] Extensible for other gui. Since the core principles dont care about UI.
- [ ] Async Handling and server-side integration.

### Inspiration

I was inspired by alot of We frameworks out there. You can see influences of React in the `Component`, `State` and `Props` traits.
You can also see inspirations from `yew` and other similar frameworks to provide a seemless macro.
I also want to capture the declarativeness and reactiveness seen in alpine.js. Expect to see more of that direction, eg. with transitions.

#### Demo examples

> This API will certainly change.

#### Prerequisite:

You need need to have `rust`, `cargo` and `trunk` installed.

License: MIT
