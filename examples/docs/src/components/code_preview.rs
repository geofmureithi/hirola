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
fn highlight_code<'a>(_example_name: &'a str) -> Box<dyn Fn(&Dom) -> () + 'a> {
    let cb = move |node: &Dom| {
        let element = node
            .inner_element()
            .as_ref()
            .clone()
            .unchecked_into::<Element>();
        highlightElement(element);
    };
    Box::new(cb)
}

#[component]
pub fn CodePredom<T: AsRef<str>>(code: T, file: T) -> Dom {
    let file = file.as_ref();
    let code = code.as_ref().to_owned();
    html! {
        <pre class="text-sm my-2 p-2" mixin:identity=&highlight_code(file)>
            <code>{code.render()}</code>
        </pre>
    }
}
