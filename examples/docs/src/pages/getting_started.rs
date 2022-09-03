use hirola::prelude::*;

use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;

const INDEX: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
  </head>
</html>
"#;

pub fn getting_started_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
          <SeoTitle title={"Getting Started | Hirola"} />

            <h1>"Prerequisites"</h1>
            <p>
              "Before getting started with"
              <code class="one-liner">"hirola"</code>
              " we are going to assume that you have the following tools installed:"
            </p>
            <ul class="ml-4">
              <li>"Rust"</li>
              <li>"Cargo"</li>
              <li>"Trunk"</li>
            </ul>
            <h1>"Getting Started"</h1>
            <p>"We are going to create a simple counter program."</p>
            <code class="block one-liner my-1 py-1">
              "cargo new counter"
            </code>
            <p>"With a new project, we need to create an index file which is the entry point and required by trunk"</p>
            <code class="block one-liner my-1 py-1">
              "cd counter"
            </code>
            <p>"Create an "<b>"index.html"</b>" in the root of counter. Add the contents below"</p>

              <CodePreview code={INDEX} file="index.html" />
              <p>"Lets add some code to "<b>"src/main.rs" </b></p>
              <CodePreview code={include_str!("../../../counter/src/main.rs")} file="src/main.rs" />
              <p>"Now lets run our project"</p>
              <code class="block one-liner my-1 py-1">
                "trunk serve"
              </code>
              <p>"You should be able to get counter running."</p>
              <p class="text-xs"><span>"Try it out"</span></p>
              <div class="demo">
              {
                  let count = Signal::new(0);
                  html! {
                      <div>
                        <button on:click={count.mut_callback(|c, _| c + 1)}>"Increment"</button>
                        <span class="ml-1">{count.get()}</span>
                      </div>
                    }
                }
              </div>
              <p>"We can also test our project using wasm-pack"</p>
              <code class="block one-liner my-1 py-1">
                "wasm-pack test --node"
              </code>
        </div>
    }
}
