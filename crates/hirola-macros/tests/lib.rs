use hirola_macros::html;
use hirola::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// fn entry(entry: u8) -> String {
//     html_to_string! {
//         <li>{entry}</li>
//     }
// }

#[wasm_bindgen_test]
fn test() {
    let world = "planet";

    assert_eq!(
        html! {
            <p>{world}</p>
        },
        TemplateResult::<DomNode>::empty()
    );
}
