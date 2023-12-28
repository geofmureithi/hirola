#![allow(unused_variables)]
use hirola::dom::app::App;
use hirola::dom::dom_test_utils::next_tick;
use hirola::dom::Dom;
use hirola::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{Element, Node};

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> Node {
    let doc = web_sys::window().unwrap().document().unwrap();

    doc.create_element("div").unwrap().into()
}

fn inner_html(element: &Node) -> String {
    let element = element.dyn_ref::<Element>().unwrap();
    element.inner_html()
}

#[wasm_bindgen_test]
fn router_pushes() {
    let mut app: App<()> = App::new(());
    app.route("/", |_| {
        html! {
            <main>"Main"</main>
        }
    });
    app.route("/page", |_| {
        html! {
            <main>"Page"</main>
        }
    });
    let router = app.router().clone();
    let node = body();
    app.mount_to(&node);
    assert_eq!("<main>Main</main>", inner_html(&node));
    router.push("/page");

    next_tick(move || {
        assert_eq!("<main>Page</main>", inner_html(&node));
    });
}

#[wasm_bindgen_test]
fn app_renders() {
    let mut app: App<()> = App::new(());
    fn test_app(app: &App<()>) -> Dom {
        html! {
            <span>"Test"</span>
        }
    }
    let node = &body();
    app.route("/", test_app);
    app.mount_to(node);
    assert_eq!("<span>Test</span>", inner_html(node));
}

#[wasm_bindgen_test]
fn router_renders() {
    let mut app: App<()> = App::new(());
    app.route("/", |_| {
        html! {
            <main>"Main"</main>
        }
    });
    let node = &body();
    app.mount_to(node);
    assert_eq!("<main>Main</main>", inner_html(node));
}
