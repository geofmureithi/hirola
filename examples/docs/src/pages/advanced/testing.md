---
title: Testing an app built with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: Testing on hirola is based on wasm-bindgen-test.
draft: false
---

# Testing

Testing on hirola is based on wasm-bindgen-test.

> The wasm-bindgen-test crate is an experimental test harness for Rust programs compiled to wasm.[→ Read more about testing on wasm32-unknown-unknown with wasm-bindgen-test](https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html)

## Example

A testing example can be seen in the counter example

```rust
use hirola::prelude::*;
fn counter() -> Dom {
    let count = Mutable::new(0);
    html! {
        <div>
            <button on:click=count.mut_callback(|c, _| c + 1)>"Increment"</button>
            <span>{count}</span>
        </div>
    }
}
fn main() {
    hirola::render(counter()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn counter_renders() {
        let res = hirola::render_to_string(counter);
        assert_eq!("<div><button>Increment</button><span>0</span></div>", &res);
    }
}
```

Tests can be run with wasmpack

`wasm-pack test --node`

Testing is still a work in progress
