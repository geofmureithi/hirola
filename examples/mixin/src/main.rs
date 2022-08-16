use hirola::prelude::*;
use web_sys::Element;

/// Mixin that controls tailwind opacity based on a bool signal
fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        if *signal.get() {
            element.class_list().add_1("opacity-100").unwrap();
            element.class_list().remove_1("opacity-0").unwrap();
        } else {
            element.class_list().add_1("opacity-0").unwrap();
            element.class_list().remove_1("opacity-100").unwrap();
        }
    };
    Box::new(cb)
}

fn mixin_demo(_app: &HirolaApp) -> Dom {
    let is_shown = Signal::new(true);
    let toggle = is_shown.mut_callback(|show, _e| !show);

    html! {
        <div
            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
            <div
                class="h-64 w-64 block bg-blue-900 rounded-md"
                mixin:transition=&opacity(&is_shown)/>
            <button
                class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
                on:click=toggle>
                "Click Me"
            </button>
        </div>
    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", mixin_demo);
}
