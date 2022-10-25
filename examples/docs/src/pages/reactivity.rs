use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;
use web_sys::{Event, HtmlInputElement, window};
use wasm_bindgen::JsCast;

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

                  <br/>
                  <h2>"Signal"</h2>
                  <p>"Signal is pretty similar to useState in react, Alpine.reactive or ref in Vuejs. Hirola make a data reactive by wrapping the value with a"
                    <code class="one-liner">
                    "Signal"
                    </code>
                    ". One way you can alter the data inside it is by calling "
                    <code class="one-liner">
                      ".set(new_value)"
                    </code>
                    " method that will take a new value and overwriting the value inside it. Inside the "
                    <code class="one-liner">
                    "html! {}"
                    </code>
                    ", you can get access the value inside the signal by calling"
                    <code class="one-liner">
                    ".get()"
                    </code>
                    "method."
                  </p>

                  <br/>
                  <h3>"Example"</h3>
                  <CodePreview
                  code=r##"use hirola_core::prelude::*;
use web_sys::{Event, HtmlInputElement, window};

fn home(app: _HirolaApp) -> Dom {
  //Define a new reactive state
  let state = Signal::new(String::new());

  //Define a callback to change the state
  let change_state = state.callback(move |st, _e: Event| {
    let el = window().unwrap().document().unwrap().get_element_by_id("username").unwrap();
    let input = el.dyn_ref::<HtmlInputElement>().unwrap();
    let input = input.value();

    st.set(input);
  });

  html! {
    <div>
      //Accessing the reactive value
      <p>{state.get().clone()}</p>
      <input type="text" id="username" on:input=change_state />
    </div>
  }
}"##
                  file="main.rs"
                  />
                  <p>"Try it out"</p>
                  <div class="demo">
                  {
                      let demostate = Signal::new(String::new());

                      let change_state = demostate.callback(move |st, _e: Event| {
                        let el = window().unwrap().document().unwrap().get_element_by_id("demo-username").unwrap();
                        let input = el.dyn_ref::<HtmlInputElement>().unwrap();
                        let input = input.value();
                    
                        st.set(input);
                      });

                      html! {
                        <div>
                          //Accessing the reactive value
                          <p>{demostate.get().clone()}</p>
                          <input type="text" id="demo-username" on:input=change_state />
                        </div>
                      }
                  }
                  </div>

                  <br/>
                  <h2>"Subscribing Signal"</h2>
                  <p>"In some cases, you might want to listen for any changes on your "
                    <code class="one-liner">
                    "Signal"
                    </code>
                  ". You can make this via subscribing using"
                    <code class="one-liner">
                    "create_effect"
                    </code>
                  " function."
                  </p>

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
