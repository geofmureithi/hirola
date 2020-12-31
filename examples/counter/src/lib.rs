#![recursion_limit="256"]
use hirola;
use hirola::{Component, Dom, Node as DomNode, State };
use std::sync::Arc;
extern crate typed_html;
use typed_html::dom::Node;
use typed_html::html;
use hirola::events::*;

use hirola::wasm_bindgen::prelude::*;


#[derive(Default)]
struct Count {
    counter: i32,
}

impl Count {
    fn decrement(&self) {
        //TODO
    }
}

impl State for Count {}

struct Counter;


impl Component<Option<i32>, Count> for Counter {
    fn render(&mut self, state: &Arc<Count>) -> Dom {
        let mut doc = html!(
            <div class="flex w-full h-full">
                <div class="custom-number-input h-10 w-32">
                    <label for="custom-input-number" class="w-full text-gray-700 text-sm font-semibold">"Counter Input"
                    </label>
                    <div class="flex flex-row h-10 w-full rounded-lg relative bg-transparent mt-1">
                        <button 
                            onclick={ |_event| &state.decrement() } 
                            data-action="decrement" 
                            class="bg-gray-300 text-gray-600 h-full w-20 rounded-l cursor-pointer outline-none">
                            <span class="m-auto text-2xl font-thin">"−"</span>
                        </button>
                        <input class="outline-none text-center w-full bg-gray-300 font-semibold text-md flex items-center text-gray-700  outline-none" data-name="custom-input-number" value={state.counter.to_string()}></input>
                        <button data-action="increment" class="bg-gray-300 text-gray-600  h-full w-20 rounded-r cursor-pointer">
                            <span class="m-auto text-2xl font-thin">"+"</span>
                        </button>
                    </div>
                </div>
            </div>
        : DomNode);
        let vdom = doc.vnode();
        DomNode::build(&state, vdom).into_dom()
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    hirola::mount(&mut Counter, None);
    Ok(())
}
