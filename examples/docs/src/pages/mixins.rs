use std::fmt::format;
use std::str::FromStr;

use hirola::prelude::mixins::text;
use hirola::prelude::*;
use web_sys::Element;

use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;

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

pub fn mixins_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
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
      file_name="main.rs"
      />
      <div class="demo">
                {


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
                  }

        </div>


        </div>
    }
}

pub fn inner_mixins(_app: &HirolaApp) -> Dom {
    let router: &Router = _app.data().unwrap();
    let params = router.params().get();
    let params = params.params.clone();
    let mixin = params.get("mixin").cloned().unwrap_or(format!("404"));
    let mixin = InbuiltMixin::from_str(&mixin).unwrap();
    html! {
        <div>
            <h1>"Mixin"</h1>
            <p>"Install blah blah "<code mixin:text=&text(&format!("{:?}", mixin))></code></p>
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
