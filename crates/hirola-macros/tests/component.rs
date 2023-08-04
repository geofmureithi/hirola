use hirola::prelude::*;
use hirola_macros::html;

#[component]
fn MyComponent() -> Dom {
    let world = "planet";
    html! {
        <p>{world}</p>
    }
}

#[component]
fn MyComponentWithProps(world: &'static str) -> Dom {
    html! {
        <p>{world}</p>
    }
}

#[test]
fn it_renders_component() {
    let result = render_to_string({
        html! {
            <>
                <MyComponent />
            </>
        }
    });
    assert_eq!("<p>planet</p>", result);
}

#[test]
fn it_renders_component_with_props() {
    let result = render_to_string({
        html! {
            <>
                <MyComponentWithProps world="hirola" />
            </>
        }
    });
    assert_eq!("<p>hirola</p>", result);
}
