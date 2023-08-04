### Installation Example

lib.rs

```rust
use std::fmt::Display;
use hirola::prelude::*;
use hirola::signal::Mutable;

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    html! {
        <>
            <button on:click=decrement>"-"</button>
            <span>{count}</span>
            <button on:click=increment>"+"</button>
        </>
    }
}

fn main() {
    let root = render(counter()).unwrap();
    // We prevent the root from being dropped
    std::mem::forget(root);
}
```

index.html

```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
    <link
      href="https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css"
      rel="stylesheet"
    />
  </head>
</html>
```

```toml

[package]
name = "counter"
version = "0.1.0"


[dependencies]
hirola = "0.3"
console_error_panic_hook = "0.1"
log = "0.4"
console_log = "0.2"
```

Start using

```sh
$> trunk serve
```
