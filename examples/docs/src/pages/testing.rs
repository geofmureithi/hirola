use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use crate::App;
use hirola::prelude::*;

pub fn testing_page(_app: &App) -> Dom {
    html! {
        <div>
            <SeoTitle title="Testing | Hirola"/>
            <h1>"Testing"</h1>
            <p>"Testing on hirola is based on wasm-bindgen-test."</p>
            <blockquote>
                <p>
                    "The wasm-bindgen-test crate is an experimental test harness for Rust programs compiled to wasm."
                    //rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html">"→ Read more about testing on wasm32-unknown-unknown with wasm-bindgen-test"</a>
                    <a href="https://rustwasm.github.io/wasm-bindgen/wasm-bindgen-test/index.html">
                        "→ Read more about testing on wasm32-unknown-unknown with wasm-bindgen-test"
                    </a>
                </p>
            </blockquote>
            <h2>"Example"</h2>
            <p>"A testing example can be seen in the counter example"</p>
            <CodePreview code=include_str!("../../../counter/src/main.rs") file="src/main.rs"/>
            <p>"Tests can be run with wasmpack"</p>
            <code class="block one-liner my-1 py-1">"wasm-pack test --node"</code>
            <p>"Testing is still a work in progress"</p>
        </div>
    }
}
