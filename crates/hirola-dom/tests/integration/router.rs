use hirola::prelude::*;
use hirola_core::dom_test_utils::next_tick;
use hirola_core::prelude::router::Router;
use hirola_dom::Dom;
use hirola_dom::app::router::Router;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Clone)]
struct AppState {
    // ... fields and methods for your application state ...
}

fn home_page(_: &App<AppState>) -> Dom {
    Dom::text("Home")
}

fn about_page(_: &App<AppState>) -> Dom {
    Dom::text("About")
}

fn not_found_page(_: &App<AppState>) -> Dom {
    Dom::text("NotFound")
}

fn user(_: &App<AppState>) -> Dom {
    Dom::text("User")
}

fn body() -> Dom {
    Dom::fragment()
}

// Helper function to set up a test instance of Router
fn create_test_router() -> Router<AppState> {
    let mut router = Router::new();
    router.insert("/", home_page);
    router.insert("/about", about_page);
    router.insert("/users/:id", user);
    router.set_not_found(not_found_page);
    router.push("/");
    router
}

#[wasm_bindgen_test]
fn test_new_router_is_empty() {
    let router = Router::<AppState>::new();
    assert!(router.current_params().is_empty());
}
#[wasm_bindgen_test]
fn test_router_insert_and_render() {
    let router = create_test_router();

    let app = App::new(AppState {});
    let body = &body();
    let home_dom = (router.handler().at("/").unwrap().value)(&app);
    let rendered = router.clone().render(&app, &body);
    assert_eq!(rendered.inner_html(), home_dom.inner_html());
    router.push("/about");

    let about_dom = (router.handler().at("/about").unwrap().value)(&app);
    next_tick(move || {
        assert_eq!(rendered.inner_html(), about_dom.inner_html());
    })
}

#[wasm_bindgen_test]
fn test_router_current_params() {
    let router = create_test_router();
    router.push("/users/42");
    let params = router.current_params();
    assert_eq!(params.get("id"), Some(&"42".to_string()));

    router.push("/about");
    let params = router.current_params();
    assert!(params.is_empty());
}

#[wasm_bindgen_test]
fn test_router_push_and_render() {
    let router = create_test_router();
    let app = App::new(AppState {});
    let body = body();
    router.push("/about");
    let about_dom = (router.handler().at("/about").unwrap().value)(&app);
    assert_eq!(
        router.clone().render(&app, &body).inner_html(),
        about_dom.inner_html()
    );

    router.push("/");
    let home_dom = (router.handler().at("/").unwrap().value)(&app);
    assert_eq!(
        router.render(&app, &body).inner_html(),
        home_dom.inner_html()
    );
}
#[wasm_bindgen_test]
fn test_router_not_found() {
    let router = create_test_router();
    let app = App::new(AppState {});

    router.push("/non_existent_route");
    let not_found_dom = not_found_page(&app);
    assert_eq!(
        router.render(&app, &body()).inner_html(),
        not_found_dom.inner_html()
    );
}
