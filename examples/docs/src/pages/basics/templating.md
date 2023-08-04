---
title: Templating with hirola.
date: '2023-01-21'
tags: ['rust', 'hirola', 'basics', 'starter']
summary: We are going to look at how hirola handles iteration and conditional flow
draft: false
---

# Templating

Hirola uses rsx which is an implementation of jsx in rust. This also means it inherits all the caveats.

## Iteration

Looping through an array of values is important to any framework. Frameworks like react provide a key mechanism to improve on the rerendering.

## Basic

If you are iterating over a non-signal iterator, you can use the normal for-loop

### Example

```rust
{for i in 0..5 {
  html! {
      <ul>
          <li>{i}</li>
      </ul>
  }
}}
```

<div class="demo">
- 0

- 1

- 2

- 3

- 4
</div>

## With Signals

Sometimes, you are working with a signal and want to react to changes on the ui. You can use Keyed and Indexed

### Keyed

```rust
todo!();
```

### Indexed

```rust
    <ul>
        {colors
            .signal_vec()
            .render_map(|item| {
                html! { <li>{item}</li> }
            })
        }
    </ul>
```

## Components

One can write components as functions starting with uppercase and add a `component` proc attribute

```rust
#[component]
fn Todo(router: Router) -> Dom {
    html! {
        <main/>
    }
}
html! {
  <Todo router={router} />
}
```
