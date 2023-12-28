use hirola::{
    dom::{
        effects::{attr_use::attr_signal, prelude::*},
        mount, Dom,
    },
    prelude::*,
};

use wasm_bindgen::JsCast;
use web_sys::Element;

fn mixin_demo() -> Dom {
    let is_shown = Mutable::new(true);
    let duration = Mutable::new(800u64);
    html! {
        <form
            use:future=async {
                // similar to on mount
            }
            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000"
        >
            <span x:html="<strong>calebporzio</strong>" />
            <input
                type="range"
                bind:value=&duration
                use:signal=attr_signal("data-value", duration.signal())
                max=1000
                step=100
            />
            <div class="base">
                <h1>{duration}</h1>

                <p>"Shown: "{is_shown.clone()}</p>

                <button
                    class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
                    on:click=move |e| {
                        e.prevent_default();
                        *is_shown.lock_mut() = !is_shown.get();
                    }
                >
                    "Click Me"
                </button>
            </div>
        </form>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount(mixin_demo()).unwrap();
}
