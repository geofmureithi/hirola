use hirola::prelude::*;
use std::f64;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::Event;
use web_sys::HtmlInputElement;

fn draw_canvas(_: &HirolaApp) -> TemplateResult<DomNode> {
    create_effect(move || {
        let link = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("smile");
        //.unwrap();
        // let canvas = link.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        // canvas.set_width(640);
        // canvas.set_height(480);
    });

    html! {
           <canvas width={200} id="smile"/>
    }
}

fn main() {
    let app = HirolaApp::new();

    app.mount("body", draw_canvas);
}
