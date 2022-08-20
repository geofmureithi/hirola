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
fn highlight_code<'a>(example_name: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        unsafe {
            highlightElement(element);
        };
    };
    Box::new(cb)
}

pub fn CodePreview(code: &'static str, file_name: &'static str) -> Dom {
    html! {
        <pre class="text-sm my-2 p-2" mixin:code=&highlight_code(file_name)><code>{code}</code></pre>
    }
}
