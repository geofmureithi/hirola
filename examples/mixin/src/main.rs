use hirola::{
    dom::{render_to, Dom},
    prelude::*,
};
use wasm_bindgen::JsCast;
use web_sys::Element;

fn x_html<'a>(text: &'a str) -> Box<dyn Fn(&Dom) -> () + 'a> {
    let cb = move |node: &Dom| {
        let dom = node.inner_element();
        let element = dom.dyn_ref::<Element>().unwrap();
        element.set_inner_html(&format!("{text}")); // Remember to escape this.
    };
    Box::new(cb)
}

fn mixin_demo() -> Dom {
    let raw = "<strong>calebporzio</strong>";
    let is_shown = Mutable::new(true);
    let toggle = is_shown.callback_with(|show, _e| *show.lock_mut() = !show.get());

    html! {
        <div
            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
            <div class="base">
                <h1>"Styled"</h1>
                <p mixin:identity=&x_html(raw) />
                <button
                    class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
                    on:click=toggle>
                    "Click Me"
                </button>
            </div>


        </div>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let _res = render_to(mixin_demo(), &body).unwrap();
}
