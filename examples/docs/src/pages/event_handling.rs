use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn event_handling_page(_app: &HirolaApp) -> Dom {
    html! {
          <div>
              <h1>"Event Handling"</h1>
              <p>"Hirola uses an "<code>"on:<event>"</code>" binding style"</p>
              <h2>"Example"</h2>
              <CodePreview
               code=r#"let clicked = Signal::new(false);
html! {
  <div>
    <button on:click={clicked.mut_callback(|c, _| !c)}>"Click Me"</button>
    <span>{format!("Clicked? {}", clicked.get())}</span>
  </div>
}"#
    file_name="src/main.rs" />

              <div class="demo">
                  {
                      let clicked = Signal::new(false);
                      html! {
                          <div>
                            <button on:click={clicked.mut_callback(|c, _| !c)}>"Click Me"</button>
                            <span class="ml-1">{format!("Clicked? {}", clicked.get())}</span>
                          </div>
                        }
                    }
                  </div>
            <blockquote class="my-2">
                  <p>"Hirola uses mounts events to web_sys::Element under the hood, so you should be able to use any valid eventhandler."
                    <a href="https://developer.mozilla.org/en-US/docs/Web/Events">"→ Read more about Events on MDN"</a>
                  </p>
                </blockquote>

          </div>
      }
}
