# Hirola


[![Latest Version](https://img.shields.io/crates/v/hirola.svg)](https://crates.io/crates/hirola)
[![Build Status](https://travis-ci.org/geofmureithi/hirola.svg?branch=master)](https://travis-ci.org/geofmureithi/hirola)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

 **Hirola** is an opinionated web framework for that is focused on simplicity and predicatability.

Here is a simple example: 

```rust
use hirola::prelude::*;
use std::sync::Arc;

#[derive(Default)]
struct Count {
    counter: Reactive<i32>,
    display: Reactive<bool>,
    non_reactive: String
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
            <div data-x-show={&state.display} class="flex w-full h-full">
                <div class="h-10 w-32">
                    <div class="flex flex-row h-10">
                        <button 
                            // data-x-transition={&state.animation.button} Coming soon
                            // data-action="decrement" Comments allowed 
                            onclick={ |_event|  &state.decrement(); }
                            class="bg-gray-300">
                            "+"
                        </button>
                        <input 
                            class="outline-none text-center"
                            value={&state.counter}
                        />
                        <button 
                            data-action="increment"
                            onclick={ |_event| &state.increment(); }
                            class="bg-gray-300">
                            "+"
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
```

### Goals
- [x] Write code that is declarative and easy to follow.
- [x] Follow Alpine-ish kind of Reactive and declarative style.
- [ ] Extensible for other gui. Since the core principles dont care about UI.
- [ ] Async Handling and server-side integration.

### Inspiration
I was inspired by alot of We frameworks out there. You can see influences of React in the `Component`, `State` and `Props` traits.
You can also see inspirations from `yew` and other similar frameworks to provide a seemless macro.
I also want to capture the declarativeness and reactiveness seen in alpine.js. Expect to see more of that direction, eg. with transitions.

#### Demo examples
- counter ToDO

PS
> The above example doesnt work. You can try the `develop` branch though.

> This API will certainly change.

#### Prerequisite:

You need need to have `rust` and `cargo` installed.
For the counter example, you need `node.js` and `npm` installed.

Note:
> The above example doesnt work yet.. The counter example in the develop branch 

> `cargo web` doesnt work possibly because of wasm-bindgen


License: MIT