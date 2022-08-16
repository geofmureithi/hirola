# Hirola

[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Build Status](https://travis-ci.org/geofmureithi/hirola.svg?branch=master)](https://travis-ci.org/geofmureithi/hirola)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

**Hirola** is an opinionated web framework for that is focused on simplicity and predictability.

## Goals

1. Keep it simple. Most Rust web frameworks have a huge learning curve and verbose syntaxes. We yearn to minimize these.
2. Make it easy to read, extend and share code. Mixins and components are kept simple and macro-free.
3. No Context. You can choose passing props down, and/or use the global-state if routing. You can write hook-like functions though.
4. Familiality. Uses rsx which is very similar to JSX.

Here is a simple example:

```rust
use hirola::prelude::*;

fn counter(_: &HirolaApp) -> Dom {
    let state = Signal::new(99);
    let decerement = state.mut_callback(|count, _| *count - 1);
    let incerement = state.mut_callback(|count, _| *count + 1);

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

### Mixins

Mixins are hirola's way of extending functionality and following DRY principles. Here is an example:

```rust
// Mixin that controls tailwind opacity based on a bool signal
fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
   let cb = move |node: DomNode| {
       let element = node.unchecked_into::<Element>();
       if *signal.get() {
           element.class_list().add_1("opacity-100").unwrap();
           element.class_list().remove_1("opacity-0").unwrap();
       } else {
           element.class_list().add_1("opacity-0").unwrap();
           element.class_list().remove_1("opacity-100").unwrap();
       }
   };
   Box::new(cb)
}
```

You can now use you mixin on a dom node eg:

```rust
html! {
    <div class="bla blah" mixin:transition=opacity(&display)/>
}
```

Since you are passing a signal, you can now manipulate the signal to change the opacity.

Mixins run in namespaces, eg the one above is run in `transition` namespace.
This allows you to only run specific mixins. The inbuilt form mixins can only be run in `mixin:form` namespace.

## Ecosystem

Here are some extensions for hirola:

1. [Form](https://crates.io/crates/hirola-form)

### Milestones

| Status | Goal                                                                      | Labels        |
| :----: | :------------------------------------------------------------------------ | ------------- |
|   ✔    | Write code that is declarative and easy to follow                         | `ready`       |
|   ✔    | Allow extensibility via mixins                                            | `ready`       |
|   ❌   | [Standardize Components](https://github.com/geofmureithi/hirola/issues/1) | `inprogress`  |
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
