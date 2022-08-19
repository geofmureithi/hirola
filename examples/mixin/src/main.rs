use std::fmt::Display;

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
    let raw = "<strong>calebporzio</strong>";
    let is_shown = Signal::new(true);
    let toggle = is_shown.mut_callback(|show, _e| !show);

    html! {
        <div

            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
            <style>{
                style! {
                    ".base" {
                        "position": "fixed";
                        "top": "0";
                        "left": "0";
                        "width": "100vw";
                        "height": "100vh";
                        "color": "#ffffff";
                    }

                    ".base > h1" {
                        "width": "max-content";
                        "margin-left": "auto";
                        "margin-right": "auto";
                        "margin-top": {if *is_shown.get() { "5px" } else { "10px" }};
                    }

                    @media "(orientation: landscape)" {
                        ".base" {
                            "background-color": "#0366d6";
                        }
                    }

                    @media "(orientation: portrait)" {
                        ".base" {
                            "background-color": "#d73a49";
                        }
                    }
                }
            }</style>
            <div class="base">
                <h1>{"Styled"}</h1>
                <div
                    class="h-64 w-64 block bg-blue-900 rounded-md"
                />
                <p mixin:text=&text(&is_shown.get()) />
                <p mixin:html=&rhtml(raw) />
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
    let app = HirolaApp::new();
    app.mount("body", mixin_demo);
}
