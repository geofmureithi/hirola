mod async_handling;
mod event_handling;
mod extending;
mod form;
mod getting_started;
mod mixins;
mod reactivity;
mod router;
mod ssr;
mod state;
mod templating;
mod testing;

use hirola::prelude::*;

pub use async_handling::async_page;
pub use event_handling::event_handling_page;
pub use extending::extending_page;
pub use form::forms_page;
pub use getting_started::getting_started_page;
pub use mixins::inner_mixins;
pub use mixins::mixins_page;
pub use reactivity::reactivity_page;
pub use router::router_page;
pub use ssr::ssr_page;
pub use state::state_page;
pub use templating::templating_page;
pub use testing::testing_page;

use crate::components::code_preview::CodePreview;
use crate::App;

pub fn home(_: &App) -> Dom {
    html! {
        <div>
            <h1>"What is Hirola?"</h1>
            <p><strong>"Hirola"</strong>" is an un-opinionated Rust web framework that is focused on simplicity and predictability."</p>
            <h2>"Goals"</h2>
            <ul>
              <li>"Keep it simple. Most Rust web frameworks have a huge learning curve and verbose syntaxes. We yearn to minimize these."</li>
              <li>"Make it easy to read, extend and share code. Mixins and components are kept simple and macro-free."</li>
              <li>"No Context. You can choose passing props down, and/or use the global-state if routing. You can write hook-like functions though."</li>
              <li>"Familiality. Uses rsx which is very similar to JSX."</li>
            </ul>
            <h2>"Example"</h2>
            <CodePreview
             code=
r#"use hirola::prelude::*;

fn counter(_: &HirolaApp) -> Dom {
  let state = Signal::new(99);
  let decerement = state.mut_callback(|count, _| *count - 1);
  let incerement = state.mut_callback(|count, _| *count + 1);

  html! {
      <div class="flex flex-row h-10">
          <button on:click=decerement>"-"</button>
          <input value=state.get() disabled/>
          <button on:click=incerement>"+"</button>
      </div>
  }
}

fn main() {
  let window = web_sys::window().unwrap();
  let document = window.document().unwrap();
  let body = document.body().unwrap();

  let app = HirolaApp::new();
  app.mount(&body, counter);
}"#
      file="main.rs"/>

              <div class="demo">
                {
                  let state = Mutable::new(99);
                  let decrement = state.update_with(|count, _| *count - 1);
                  let increment = state.update_with(|count, _| *count + 1);

                  html! {
                      <div class="flex flex-row h-10">
                          <button on:click=decrement>"-"</button>
                          <input class="w-12" value=state.get() disabled/>
                          <button on:click=increment>"+"</button>
                      </div>
                  }
                }
              </div>
            <h2>"Features"</h2>
            <ul>
              <li>
              <p><strong><code>"serde"</code></strong> "—  Enables serialization of state"</p>
              </li>
              <li>
              <p><strong><code>"ssr"</code></strong> "—  Enables server side rendering 🚧"</p>
              </li>
              <li>
              <p><strong><code>"router"</code></strong> "—  Enables Isomorphic Routing"</p>
              </li>
              <li>
              <p><strong><code>"form"</code></strong> "—  Enables form mixins and utilities 🚧"</p>
              </li>
            </ul>

        </div>
    }
}
