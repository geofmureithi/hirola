---
title: Event Handling with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: We are going to learn how to handle event handling using hirola
draft: false
---

# Event Handling

Hirola uses an `on:<event>` binding style

> Hirola uses mounts events to web_sys::Element under the hood, so you should be able to use any valid eventhandler.[→ Read more about Events on MDN](https://developer.mozilla.org/en-US/docs/Web/Events)

## Example

```rust
html! {
  <button
      on:click=|e| {
        let window = web_sys::window().unwrap();
        window.alert_with_message("Hello from Hirola!");
      }>
      "Click Me"
  </button>
}

```
