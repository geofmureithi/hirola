use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn reactivity_page(_app: &HirolaApp) -> Dom {
    html! {
                <div>
                    <SeoTitle title={"Reactivity | Hirola"} />
                    <h1>"Reactivity"</h1>
                    <p>
                    r#"Hirola offers reactivity via a primitive called signal and an effect called create_effect. Once a signal is updated, these changes are propagated to the dom."#
                    </p>
                    <blockquote>
                    <p>"Hirola uses a fork of maple(now sycamore) reactivity engine under the hood to provide these functions."
                      <a href="https://sycamore-rs.netlify.app/docs/basics/reactivity">"→ Read more about sycamore reactivity primitives"</a>
                    </p>
                  </blockquote>
                  <h2>"Reactive Signal"</h2>
                  <CodePreview
                  code=
                      "use hirola_core::prelude::*;
let state = Signal::new(0);
assert_eq!(*state.get(), 0);
              
state.set(1);
assert_eq!(*state.get(), 1);"
                    file="main.rs" />


                  <p>"Signal is pretty similar to useState in react or Alpine.reactive"</p>
                  <h2>"Subscribing"</h2>
                  <p>"Subscribing is done via create_effect"</p>
                  <CodePreview
                 code=
"use hirola_core::prelude::*;
let state = Signal::new(0);
assert_eq!(*state.get(), 0);
create_effect(move || {
let new_value = state.get();
// do something with new value
})
/// later
state.set(1);
"
    file="main.rs" />

                </div>
            }
}
