use hirola::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
extern "C" {
    /// Highlight.js
    #[wasm_bindgen(js_namespace = hljs)]
    fn highlightElement(element: Element);
}

/// mixin to highlight code
fn highlight_code<'a>(_example_name: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        highlightElement(element);
    };
    Box::new(cb)
}

#[component]
pub fn CodePreview<'a>(code: &'a str, file: &'a str) -> Dom {
    let file = file.to_string();
    let code = code.to_string();
    html! {
        <pre class="text-sm my-2 p-2" mixin:code=&highlight_code(&file)><code>{code.clone()}</code></pre>
    }
}
