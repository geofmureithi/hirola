use hirola::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::Event;
use web_sys::HtmlInputElement;

fn App() -> TemplateResult<DomNode> {
    let reff = NodeRef::new();
    if reff.try_get::<DomNode>().is_some() {
        let link = reff.get::<DomNode>();
        let canvas = link
            .inner_element()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        canvas.set_width(640);
        canvas.set_height(480);
        // canvas.style().set_property("border", "solid").unwrap();
    }

    html! {
           <canvas ref={reff} width={480} id="smile"/>
    }
}

fn main() {
    let mut app = HirolaApp::new();

    app.mount("body", App);
}
