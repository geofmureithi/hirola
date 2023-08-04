---
title: Server Side Rendering
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: Hirola supports basic server side rendering
draft: false
---

# Server Side Rendering

Hirola supports basic server side rendering with the feature `ssr`.

## Example
```rust
fn main(){
    let app = HirolaApp::new();
    let res = app.render_to_string(counter);
    assert_eq!("<div><button>Increment</button><span>0</span></div>", &res);
}
```