use hirola_macros::{component, html};
use hirola_ssr::{render_to_string, SsrNode};

#[component]
fn MyComponent() -> SsrNode {
    let world = "planet";
    html! { <p>{world}</p> }
}

#[component]
fn MyComponentWithProps(world: &'static str) -> SsrNode {
    html! { <p>{world}</p> }
}

#[test]
fn it_renders_component() {
    let result = render_to_string({
        html! {
            <>
                <MyComponent/>
            </>
        }
    })
    .unwrap();
    assert_eq!("<p>planet</p>", result);
}

#[test]
fn it_renders_component_with_props() {
    let result = render_to_string({
        html! {
            <>
                <MyComponentWithProps world="hirola"/>
            </>
        }
    })
    .unwrap();
    assert_eq!("<p>hirola</p>", result);
}
