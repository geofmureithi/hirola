use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn event_handling_page(_app: &HirolaApp) -> Dom {
    html! {
          <div>
          <SeoTitle title={"Event Handling | Hirola"} />
              <h1>"Event Handling"</h1>
              <p>"Hirola uses an "<code class="one-liner">"on:<event>"</code>" binding style"</p>
              <blockquote class="my-2">
                  <p>"Hirola uses mounts events to web_sys::Element under the hood, so you should be able to use any valid eventhandler."
                    <a href="https://developer.mozilla.org/en-US/docs/Web/Events">"→ Read more about Events on MDN"</a>
                  </p>
                </blockquote>
              <h2>"Example"</h2>
              <CodePreview
               code=r#"html! {
  <button
      on:click=|e| {
        let window = web_sys::window().unwrap();
        window.alert_with_message("Hello from Hirola!");
      }>
      "Click Me"
  </button>
}"#
    file_name="src/main.rs" />

              <div class="demo">
                  {

                      html! {
                          <button
                              on:click=|e| {
                                let window = web_sys::window().unwrap();
                                window.alert_with_message("Hello from Hirola!");
                              }>
                              "Click Me"
                          </button>
                      }
                    }
                  </div>


          </div>
      }
}
