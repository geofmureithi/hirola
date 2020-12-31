
### Installation Example
lib.rs
```rust
use hirola::prelude::*;
use std::sync::Arc;

#[derive(Default)]
struct Count {
    counter: Reactive<i32>,
}

impl Count {
    fn increment(&self) {
        let count = &self.counter.clone();
        count.replace_with(|x| *x + 1);
    }

    fn decrement(&self) {
        let count = &self.counter.clone();
        count.replace_with(|x| *x - 1);
    }
}

impl State for Count {}

struct Counter;

impl Component<Option<i32>, Count> for Counter {
    fn render(&mut self, state: &Arc<Count>) -> Dom {
        render! {
            <div data-x-show={true} class="flex w-full h-full">
                <div class="custom-number-input h-10 w-32">
                    <label for="custom-input-number" class="w-full text-gray-700 text-sm font-semibold">"Counter Input"
                    </label>
                    <div class="flex flex-row h-10 w-full rounded-lg relative bg-transparent mt-1">
                        <button 
                            data-action="decrement" 
                            onclick={ |_event| { &state.decrement() }}
                            class="bg-gray-300 text-gray-600 h-full w-20 rounded-l cursor-pointer outline-none">
                            <span class="m-auto text-2xl font-thin">"−"</span>
                        </button>
                        <input 
                            class="outline-none text-center w-full bg-gray-300 font-semibold text-md flex items-center text-gray-700 outline-none" data-name="custom-input-number" 
                            value={state.counter}
                        />
                        <button 
                            data-action="increment"
                            onclick={ |_event| { &state.increment() }}
                            class="bg-gray-300 text-gray-600  h-full w-20 rounded-r cursor-pointer">
                            <span class="m-auto text-2xl font-thin">"+"</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    hirola::mount(&mut Counter, None);
    Ok(())
}

```

public/index.html
```html
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8">
    <title>Hirola Counter</title>
    <link href="https://unpkg.com/tailwindcss@^2/dist/tailwind.min.css" rel="stylesheet">
  </head>
  <body>
    <script src="js/index.js"></script>
  </body>
</html>
```
In Cargo.toml, specify the crate-type to be `cdylib`
```toml

[package]
name = "counter"
version = "0.1.0"
edition = "2018"

[lib]
crate-type = ["cdylib"]


[dependencies]
hirola = "0.1.0"
console_error_panic_hook = "0.1"
log = "0.4"
console_log = "0.2"
```

package.json
```json
{
    "private": true,
    "author": "@geofmureithi",
    "name": "counter",
    "version": "0.1.0",
    "scripts": {
      "build": "rimraf dist/js && rollup --config",
      "start": "rimraf dist/js && rollup --config --watch"
    },
    "devDependencies": {
      "@wasm-tool/rollup-plugin-rust": "^1.0.0",
      "rimraf": "^3.0.2",
      "rollup": "^1.31.0",
      "rollup-plugin-livereload": "^1.2.0",
      "rollup-plugin-serve": "^1.0.1"
    }
  }
  
```
rollup.config.js
```js
import rust from "@wasm-tool/rollup-plugin-rust";
import serve from "rollup-plugin-serve";
import livereload from "rollup-plugin-livereload";

const is_watch = !!process.env.ROLLUP_WATCH;

export default {
    input: {
        index: "./Cargo.toml",
    },
    output: {
        dir: "public/js",
        format: "iife",
        sourcemap: true,
    },
    plugins: [
        rust({
            serverPath: "js/",
            debug: false,
        }),

        is_watch && serve({
            contentBase: "public",
            open: true,
        }),

        is_watch && livereload("public"),
    ],
};
```



Start using
```sh
$> yarn start
```

Build using
```sh
$> yarn build
```
