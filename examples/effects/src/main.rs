use hirola::{
    dom::{effects::prelude::*, mount, Dom},
    prelude::*,
};

fn effects_demo() -> Dom {
    let is_shown = Mutable::new(true);
    let duration = Mutable::new(800u64);
    html! {
        <form
            use:future=async {}
            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000"
        >
            // Raw html mixin
            <span x:html="<strong>calebporzio</strong>"></span>

            <input
                type="range"
                // Two way binding
                bind:value=&duration
                // Generic signal binding
                use:signal=attr_signal("data-value", duration.signal())
                max=1000
                step=100
            />
            <div class="base">
                // Directly include a reactive
                <h1>{duration}</h1>

                <p>"Shown: " {is_shown.clone()}</p>

                <button
                    class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
                    // Events are prefixed with on:
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
    mount(effects_demo()).unwrap();
}
