#![allow(unused_variables)]

use hirola::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn app_renders() {
    let app = HirolaApp::new();
    fn test_app(app: &HirolaApp) -> Dom {
        html! {
            <span>"Test"</span>
        }
    }
    let res = app.render_to_string(test_app);
    assert_eq!("<span>Test</span>", &res);
}

#[wasm_bindgen_test]
fn router_renders() {
    let mut app = HirolaApp::new();
    let mut router = Router::new();
    router.add("/", |_| {
        html! {
            <main>"Main"</main>
        }
    });
    app.extend(router);

    fn test_app(app: &HirolaApp) -> Dom {
        let router: &Router = app.data().unwrap();
        router.push("/");
        router.render(app)
    }
    let res = app.render_to_string(test_app);
    assert_eq!("<main>Main</main>", &res);
}

#[wasm_bindgen_test]
fn router_pushes() {
    let mut app = HirolaApp::new();
    let mut router = Router::new();
    router.add("/", |_| {
        html! {
            <main>"Main"</main>
        }
    });
    router.add("/page", |_| {
        html! {
            <main>"Page"</main>
        }
    });

    app.extend(router.clone());

    fn test_app(app: &HirolaApp) -> Dom {
        let router: &Router = app.data().unwrap();
        router.push("/page");
        router.render(app)
    }

    let res = app.render_to_string(test_app);
    assert_eq!("<main>Page</main>", &res);
}
