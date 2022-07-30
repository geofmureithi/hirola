use hirola::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;

fn Home() -> TemplateResult<DomNode> {
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

    app.mount("body", Home);
}
