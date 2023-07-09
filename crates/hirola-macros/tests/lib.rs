use hirola_core::ViewBuilder;
use hirola_macros::html;

#[test]
fn test() {
    let world = "planet";

    assert_eq!(
        html! {
            <p>{world}</p>
        },
        ViewBuilder::new()
    );
}
