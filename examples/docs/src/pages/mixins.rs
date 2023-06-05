use std::str::FromStr;

use hirola::prelude::mixins::{model::input, rhtml, show, text};
use hirola::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlInputElement};

use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;

fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let signal = signal.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        let signal = signal.clone();
        create_effect(signal ,move |val| {
            if val {
                element.class_list().add_1("opacity-100").unwrap();
                element.class_list().remove_1("opacity-0").unwrap();
            } else {
                element.class_list().add_1("opacity-0").unwrap();
                element.class_list().remove_1("opacity-100").unwrap();
            }
        })
    };
    Box::new(cb)
}

pub fn mixins_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <SeoTitle title={"Mixins | Hirola"} />
            <h1>"Mixins"</h1>
            <p>"Mixins are ways of sharing and extending code in hirola."</p>
            <p>"Hirola is highly inspired by Alpine.js, and mixins can be considered similar to directives"</p>
            <p>"Mixins can be very powerful in applying DRY techniques. Lets start simple and create a mixin that controls tailwinds opacity."</p>
            <h2>"Example"</h2>
            <CodePreview
             code=
r#"use web_sys::Element;
/// Mixin that controls tailwind opacity based on a bool signal
fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
  let signal = signal.clone();
  let cb = move |node: DomNode| {
      let element = node.unchecked_into::<Element>();
      let signal = signal.clone();
      create_effect(move || {
          if *signal.get() {
              element.class_list().add_1("opacity-100").unwrap();
              element.class_list().remove_1("opacity-0").unwrap();
          } else {
              element.class_list().add_1("opacity-0").unwrap();
              element.class_list().remove_1("opacity-100").unwrap();
          }
      })
  };
  Box::new(cb)
}

let is_shown = Signal::new(true);

let toggle = is_shown.mut_callback(|show, _e| !show);

html! {
    <div class="transition ease-in-out">
        <div
            class="h-64 w-64 block bg-blue-900 rounded-md my-2"
            mixin:opacity=&opacity(&is_shown)
        />
        <button on:click={toggle}>"Toggle"</button>
    </div>
}

"# 
      file="main.rs"
      />
      <div class="demo">
                // {


                //   let is_shown = Signal::new(true);
                //   let toggle = is_shown.mut_callback(|show, _e| !show);
                //     html! {
                //         <div class="transition ease-in-out">

                //           <div
                //             class="h-64 w-64 block bg-blue-900 rounded-md my-2"
                //             mixin:opacity=&opacity(&is_shown)
                //           />

                //           <button on:click=toggle>"Toggle"</button>
                //         </div>
                //       }
                //   }

        </div>


        </div>
    }
}

pub fn inner_mixins(_app: &HirolaApp) -> Dom {
    let router: &Router = _app.data().unwrap();
    let param = router.param("mixin").unwrap_or(format!("404"));
    let mixin = InbuiltMixin::from_str(&param).unwrap();
    let title = format!("Mixin - mixin:{} | Hirola", param);
    html! {
        <div>
          <SeoTitle title={&title} />
            <h1>{format!("mixin:{}", param)}</h1>
            {
              match mixin {
                  InbuiltMixin::Show => {
                    html! {
                      <div>
                        <p>"A css-powered mixin that toggles display based on a signal"</p>
                        <h2>"Example"</h2>
                        <CodePreview
                          code=
r#"let shown = Signal::new(true);
  html! {
    <div>
      <button on:click=shown.mut_callback(|c, _| !c)>"Toggle"</button>
      <span class="ml-1" mixin:show=&show(&shown)>"I am shown"</span>
    </div>
  }
"#
                          file="main.rs"
                        />
                        <div class="demo transition-all">
                        {
                            let shown = Signal::new(true);
                            html! {
                                <div>
                                  <button on:click=shown.mut_callback(|c, _| !c)>"Toggle"</button>
                                  <span class="ml-1" mixin:show=&show(&shown)>"I am shown"</span>
                                </div>
                            }
                        }
                        </div>
                      </div>
                    }
                  },
                  InbuiltMixin::Text => {
                    html! {
                      <div>
                        <p>"A text mixin that binds a signal to an element's textContent"</p>
                        <h2>"Example"</h2>
                        <CodePreview
                          code=
r#"let message = Signal::new(format!("Hello Hirola"));
let handle_change = message.mut_callback(|cur, e: Event| {
    let input = e
        .current_target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    input.value()
});
html! {
    <div>
      <span class="block" mixin:text=&text(&message) />
      <input on:keyup=handle_change value=&message.get()/>
    </div>
}
"#
                          file="main.rs"
                        />
                        <div class="demo transition-all">
                        {
                            let message = Signal::new(format!("Hello Hirola"));
                            let handle_change = message.mut_callback(|_cur, e: Event| {
                              let input = e
                                  .current_target()
                                  .unwrap()
                                  .dyn_into::<HtmlInputElement>()
                                  .unwrap();
                              input.value()
                            });
                            html! {
                                <div>
                                  <span class="block" mixin:text=&text(&message) />
                                  <input on:keyup=handle_change value=&message.get_cloned()/>
                                </div>
                            }
                        }
                        </div>
                      </div>
                    }

                  },
                  InbuiltMixin::RHtml => {
                    html! {
                      <div>
                        <p>"A mixin that allows setting raw html"</p>
                        <h2>"Example"</h2>
                        <CodePreview
                          code=
r#"let message = "<strong>Hello Hirola</strong>";
html! {
  <div>
    <span mixin:rhtml=&rhtml(message)></span>
  </div>
}
"#
                          file="main.rs"
                        />
                        <div class="demo">
                        {
                            let message = "<strong>Hello Hirola</strong>";
                            html! {
                                <div>
                                  <span mixin:rhtml=&rhtml(message)></span>
                                </div>
                            }
                        }
                        </div>
                      </div>
                    }

                  },
                  InbuiltMixin::Model => {
                    html! {
                      <div>
                        <p>"A mixin that makes two-way binding on a signal and form element"</p>
                        <h2>"Example"</h2>
                        <CodePreview
                          code=
r#"let message = Signal::new(format!("Hello Hirola"));
html! {
    <div>
      <span class="block" mixin:text=&text(&message) />
      <input mixin:model=&model_input(&message)/>
    </div>
}
"#
                          file="main.rs"
                        />
                        <div class="demo transition-all">
                        {
                            let message = Signal::new(format!("Hello Hirola"));
                            html! {
                                <div>
                                  <span class="block" mixin:text=&text(&message) />
                                  <input mixin:model=&input(&message)/>
                                </div>
                            }
                        }
                        </div>
                      </div>
                    }


                  },
                  _ => {
                    html! {
                      <p>"TODO"</p>
                    }
                  }
              }
            }
        </div>
    }
}

#[derive(Debug)]
enum InbuiltMixin {
    Show,
    Text,
    RHtml,
    Model,
    Transition,
    Ignore,
    If,
}

impl FromStr for InbuiltMixin {
    type Err = ();
    fn from_str(input: &str) -> Result<InbuiltMixin, Self::Err> {
        match input {
            "show" => Ok(InbuiltMixin::Show),
            "text" => Ok(InbuiltMixin::Text),
            "rhtml" => Ok(InbuiltMixin::RHtml),
            "model" => Ok(InbuiltMixin::Model),
            "transition" => Ok(InbuiltMixin::Transition),
            "ignore" => Ok(InbuiltMixin::Ignore),
            "if" => Ok(InbuiltMixin::If),
            _ => Err(()),
        }
    }
}
