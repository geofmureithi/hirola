use hirola::prelude::*;
use web_sys::Element;

fn x_html<'a>(text: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_inner_html(&format!("{text}")); // Remember to escape this.
    };
    Box::new(cb)
}

fn mixin_demo() -> Dom {
    let raw = "<strong>calebporzio</strong>";
    let is_shown = Mutable::new(true);
    let toggle = is_shown.update_with(|show, _e| !show);

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
                        "margin-top": {if is_shown.get() { "5px" } else { "10px" }};
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
                <h1>"Styled"</h1>
                <p mixin:html=&x_html(raw) />
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

    let app = App<S, G>::new();
    render_to(mixin_demo,&body, );
}
