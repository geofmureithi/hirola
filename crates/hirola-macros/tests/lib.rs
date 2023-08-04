use hirola::prelude::*;
use hirola_macros::html;

#[test]
fn it_works() {
    let world = "planet";
    let template = html! {
        <p>{world}</p>
    };
    let result = render_to_string(template);
    assert_eq!("<p>planet</p>", result);
}
