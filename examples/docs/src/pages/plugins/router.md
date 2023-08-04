---
title: Route handling with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: We are going to learn how to handle route handling using hirola
draft: false
---

# Router

Hirola is un-opinionated in route management. It should be pretty easy to roll out your own. To enable the inbuilt router use the feature flag `app`.

```rust
let mut app = App::new(());
app.route("/", home);
app.route("/todo/:id", todo_view);
app.mount();
```
