#![allow(unused_variables)]
use hirola::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_test::*;
use web_sys::{Element, Node};

wasm_bindgen_test_configure!(run_in_browser);

fn body() -> Node {
    let doc = web_sys::window().unwrap().document().unwrap();
    let element = doc.create_element("div").unwrap().into();
    element
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
    let node = body();
    app.mount(&node);
    assert_eq!("<main>Main</main>", inner_html(&node));
    app.router().push("/page");
    next_tick(move || {
        assert_eq!("<main>Page</main>", inner_html(&node));
    });
}

#[allow(dead_code)]
fn next_tick<F: Fn() + 'static>(f: F) {
    let a = Closure::<dyn Fn()>::new(move || f());
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback(a.as_ref().unchecked_ref())
        .unwrap();
}

#[wasm_bindgen_test]
fn app_renders() {
    let mut app: App<()> = App::new(());
    fn test_app(app: &App<()>) -> DomBuilder {
        html! {
            <span>"Test"</span>
        }
    }
    app.route("/", test_app);
    app.mount(&body());
    assert_eq!("<span>Test</span>", inner_html(&body()));
}

#[wasm_bindgen_test]
fn router_renders() {
    let mut app: App<()> = App::new(());
    app.route("/", |_| {
        html! {
            <main>"Main"</main>
        }
    });
    let view = app.mount(&body());
    assert_eq!("<main>Main</main>", inner_html(&body()));
}
