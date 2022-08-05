### Installation Example

lib.rs

```rust
use hirola::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;

fn home() -> Dom {
    let state = Signal::new(99);

    let decerement = state.reduce_callback(|count, _| *count - 1);

    let incerement = state.reduce_callback(|count, _| *count + 1);

    html! {
            <div class="grid h-screen place-items-center">

                <div class="h-10 w-32">
                    <div class="flex flex-row h-10">
                        <button
                            on:click={decerement}
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "-"
                        </button>
                        <div class="block">
                            <input
                                value={state.get()}
                                disabled
                            />
                        </div>
                        <button
                            on:click={incerement}
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "+"
                        </button>
                    </div>
                </div>
           </div>
    }
}

fn main() {
    let mut app = HirolaApp::new();
    app.mount("body", home);
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
hirola = "0.1"
console_error_panic_hook = "0.1"
log = "0.4"
console_log = "0.2"
```

Start using

```sh
$> trunk serve
```
