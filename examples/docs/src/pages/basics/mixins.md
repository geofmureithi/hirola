---
title: Mixin Handling with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: We are going to learn how to handle mixins using hirola
draft: false
---

# Mixins

## Example

Mixins allow developers to extend functionality by attaching it to a dom node.

```rust
use web_sys::Element;
/// Mixin that controls tailwind opacity based on a bool signal
fn opacity<'a>(signal: &'a Mutable<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
  let signal = signal.clone();
  let cb = move |node: DomNode| {
      let element = node.unchecked_into::<Element>();
      let signal = signal.clone();
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
