pub mod keyed;
pub mod non_keyed;

use futures_signals::signal::Mutable;
use hirola::prelude::*;
use wasm_bindgen_test::*;
use web_sys::{Document, HtmlElement, Node, Window};

wasm_bindgen_test_configure!(run_in_browser);

fn window() -> Window {
    web_sys::window().unwrap()
}

fn document() -> Document {
    window().document().unwrap()
}

/// Returns a [`Node`] referencing the test container with the contents cleared.
fn test_div() -> Node {
    if document()
        .query_selector("div#test-container")
        .unwrap()
        .is_none()
    {
        document()
            .body()
            .unwrap()
            .insert_adjacent_html("beforeend", r#"<div id="test-container"></div>"#)
            .unwrap();
    }

    let container = document()
        .query_selector("div#test-container")
        .unwrap()
        .unwrap();

    container.set_inner_html(""); // erase contents from previous test runs

    container.into()
}

#[wasm_bindgen_test]
fn hello_world() {
    let node = html! {
        <p>"Hello World!"</p>
    };

    let _ = render_to(node, &test_div());

    assert_eq!(
        &document()
            .query_selector("p")
            .unwrap()
            .unwrap()
            .outer_html(),
        "<p>Hello World!</p>"
    );
}

#[wasm_bindgen_test]
fn hello_world_noderef() {
    let p_ref = NodeRef::new();

    let node = html! {
        <p ref=p_ref> "Hello World!"</p>
    };

    let _ = render_to(node, &test_div());

    assert_eq!(
        &p_ref.get().unchecked_into::<HtmlElement>().outer_html(),
        "<p>Hello World!</p>"
    );
}

#[wasm_bindgen_test]
fn interpolation() {
    let text = "Hello Hirola!";
    let node = html! {
        <p>{text}</p>
    };

    let _ = render_to(node, &test_div());

    assert_eq!(
        document()
            .query_selector("p")
            .unwrap()
            .unwrap()
            .text_content()
            .unwrap(),
        "Hello Hirola!"
    );
}

#[wasm_bindgen_test]
fn reactive_text() {
    let count = Mutable::new(0);

    let node = html! {
        <p> { count.clone() }</p>
    };

    let _ = render_to(node, &test_div());

    let p = document().query_selector("p").unwrap().unwrap();

    assert_eq!(p.text_content().unwrap(), "0");

    count.set(1);
    non_keyed::next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "1");
    });
}

#[wasm_bindgen_test]
fn reactive_attribute() {
    let count = Mutable::new(0);

    let node = html! {
        <span attribute=count.get()/>
    };

    let _ = render_to(node, &test_div());

    let span = document().query_selector("span").unwrap().unwrap();

    assert_eq!(span.get_attribute("attribute").unwrap(), "0");

    count.set(1);
    assert_eq!(span.get_attribute("attribute").unwrap(), "1");
}

// #[wasm_bindgen_test]
// fn noderefs() {
//     let noderef = NodeRef::new();

//     let node = html! {
//         <div>
//             <input ref=noderef />
//         </div>
//     };

//     render_to(node, &test_div());

//     let input_ref = document().query_selector("input").unwrap().unwrap();

//     assert_eq!(
//         Node::from(input_ref),
//         noderef.get().unchecked_into()
//     );
// }
