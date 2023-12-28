### Installation Example

lib.rs

```rust
use hirola::prelude::*;
use hirola::dom::*;
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
    mount(counter()).unwrap();
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
hirola = "0.4"
```

Start using

```sh
$> trunk serve
```
