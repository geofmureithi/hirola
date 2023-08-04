---
title: State Management with hirola
date: '2023-01-21'
tags: ['rust', 'hirola', 'basics', 'starter']
summary: We are going to look at how hirola handles state management
draft: false
---


# State Management

Hirola allows basic state management using `app` feature.

## Getting started

```rs
let window = web_sys::window().unwrap();
let document = window.document().unwrap();
let body = document.body().unwrap();
let todos = MutableVec::new();
let mut app = App::new(todos);
/// Add routes
app.mount();
```

With that you can access the state from the current route.
