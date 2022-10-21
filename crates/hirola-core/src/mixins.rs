//! ## Mixins
//! Hirola aims to be extensible and follow DRY principles.
//! Here is an example of a mixin
//! ```rust,no_run
//! use hirola::prelude::*;
//! use web_sys::Element;
//! // Mixin that controls tailwind opacity based on a bool signal
//! fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
//!    let cb = move |node: DomNode| {
//!        let element = node.unchecked_into::<Element>();
//!        if *signal.get() {
//!            element.class_list().add_1("opacity-100").unwrap();
//!            element.class_list().remove_1("opacity-0").unwrap();
//!        } else {
//!            element.class_list().add_1("opacity-0").unwrap();
//!            element.class_list().remove_1("opacity-100").unwrap();
//!        }
//!    };
//!    Box::new(cb)
//! }
//!
//! fn mixin_demo(_app: &HirolaApp) -> Dom {
//!    let is_shown = Signal::new(true);
//!    let toggle = is_shown.mut_callback(|show, _e| !show);
//!    html! {
//!        <div
//!            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
//!            <div
//!                class="h-64 w-64 block bg-blue-900 rounded-md"
//!                mixin:transition=&opacity(&is_shown)/>
//!            <button
//!                class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
//!                on:click=toggle>
//!                "Click Me"
//!            </button>
//!        </div>
//!    }
//! }
//! fn main() {
//!     let window = web_sys::window().unwrap();
//!     let document = window.document().unwrap();
//!     let body = document.body().unwrap();
//!     let app = HirolaApp::new();
//!
//!     app.mount(&body, mixin_demo);
//! }
//! ```
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, HtmlInputElement};

use crate::{
    callback::MixinError,
    generic_node::{DomNode, GenericNode},
    prelude::{create_effect, Mixin, Signal},
};

/// A mixin that allows adding raw html
/// 
/// Note: This is a security risk if the string to be inserted might contain potentially malicious content.
/// sanitize the content before it is inserted.
/// See more: [https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML](https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML)
pub fn rhtml<'a>(text: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_inner_html(text);
    };
    Box::new(cb)
}

/// A mixin that allows adding nonsignal text
pub fn rtext<'a, D: Display>(text: &'a D) -> Box<dyn Fn(DomNode) + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_text_content(Some(&format!("{text}")));
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
pub fn text<T: Display>(text: &Signal<T>) -> Box<dyn Fn(DomNode)> {
    let signal = text.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        let signal = signal.clone();
        create_effect(move || {
            let element = element.clone();
            element.set_text_content(Some(&format!("{}", signal.get())));
        });
    };
    Box::new(cb)
}

/// Mixin that decide whether or not that element should be displayed on the screen. 
/// 
/// Note that this mixin only make the element to become hidden from the user screen. 
/// Meaning that, under the hood, `mixin:show` only controls the `display` property of that element.
/// 
/// ## Example
/// ```rust
/// use hirola::prelude::*;
/// use hirola::prelude::mixins::show;
/// pub fn show_or_not_box(_app: &HirolaApp) -> Dom {
///     let state = Signal::new(true);
/// 
///     html! {
///         <div>
///             <style>r##"
///             .box {
///                 background-color: black;
///                 color: white;
///             }
///             "##</style>
///             <div class="box" mixin:show=&show(&state)>
///                 <p>"I am Visible!"</p>
///             </div>
///             <button on:click=move |_| state.set(!*state.get())>"Change State"</button>
///         </div>
///     }
/// }
/// ```
pub fn show(shown: &Signal<bool>) -> Box<dyn Fn(DomNode)> {
    let signal = shown.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<HtmlElement>();
        let signal = signal.clone();

        create_effect(move || {
            let element = element.clone();
            let style = element.style();
            style
                .set_property("display", {
                    if *signal.get() {
                        ""
                    } else {
                        "none"
                    }
                })
                .unwrap();
        });
    };
    Box::new(cb)
}

/// Opposite to `mixin:show`, element will be display if the signal is false
pub fn nshow(shown: &Signal<bool>) -> Box<dyn Fn(DomNode)> {
    let signal = shown.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<HtmlElement>();
        let signal = signal.clone();

        create_effect(move || {
            let element = element.clone();
            let style = element.style();
            style
                .set_property("display", {
                    if *signal.get() {
                        "none"
                    } else {
                        ""
                    }
                })
                .unwrap();
        });
    };
    Box::new(cb)
}

/// Model allows 2-way binding eg between a signal and an input
pub struct Model<Node, T: 'static>(Signal<T>, PhantomData<Node>);

impl<T: Display + FromStr> Mixin for Model<HtmlInputElement, T>
where
    <T as FromStr>::Err: Debug,
{
    fn mixin(&self, ns: &str, node: DomNode) -> Result<(), MixinError> {
        if ns != "model" {
            return Err(MixinError::InvalidNamespace {
                expected: "model".to_string(),
                found: ns.to_string(),
            });
        }
        let input = {
            let node = node.clone();
            node.dyn_into::<HtmlInputElement>()
                .map_err(MixinError::NodeError)?
        };
        let signal = self.0.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let new_value = input.value().parse().unwrap();
            signal.set(new_value);
        });

        node.event("input", handler);
        input.set_value(&format!("{}", &self.0.get_untracked()));
        Ok(())
    }
}

/// Two way binding for input and signals
pub mod model {
    use super::*;
    /// Bind a [HtmlInputElement] to a [Signal<T>]
    /// 
    /// ## Example
    /// ```rust
    /// use hirola::prelude::*;
    /// use hirola::prelude::mixins::{model::input, text};
    /// pub fn about(_app: &HirolaApp) -> Dom {
    ///     let state = Signal::new(0);
    ///
    ///     html!{
    ///         <div>
    ///             <p>"user input: "<span mixin:text=&text(&state) /></p>
    ///             <input type="color" mixin:model=&input(&state) />
    ///         </div>
    ///     }
    /// }
    /// ```
    pub fn input<T>(s: &Signal<T>) -> Model<HtmlInputElement, T> {
        Model(s.clone(), PhantomData)
    }
}


///Mixin for reactive classes
/// 
/// ## Example
/// ```rust
/// use hirola::prelude::*;
/// use hirola::prelude::mixins::class;
/// use web_sys::Event;
/// pub fn yellow_or_black_box(_app: &HirolaApp) -> Dom {
///     let state = Signal::new(vec!["black-box".to_string()]);
///
///     let change_class = state.callback(move |st, _e:Event| {
///         match st.get().to_vec()[0].as_str() {
///             "black-box" => st.set(vec!["yellow-box".to_string()]),
///             "yellow-box" => st.set(vec!["black-box".to_string()]),
///             _ => panic!()
///         }
///     });
/// 
///     html! {
///         <div class="" mixin:class=&class(&state)>
///             <style>r##"
///             .black-box {
///                 background-color: black;
///                 color: white;
///             }
///             .yellow-box {
///                 background-color: yellow;
///             }
///             "##</style>
///             <button on:click=change_class>"Click to change colour"</button>
///         </div>
///     }
/// }
/// ```
pub fn class<'a>(signal: &'a Signal<Vec<String>>) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let signal = signal.clone();

    let cb = move |node: DomNode| {
        let el =node.unchecked_into::<Element>();
        let signal = signal.clone();
        let current_classes: Signal<Vec<String>> = Signal::new(vec![]);
        
        create_effect(move || {
            let mut old_classes = current_classes.get().to_vec();
            let mut new_classes = signal.get().to_vec();
            new_classes.dedup();
            let mut new_classes_clone = new_classes.clone();


            //filter the ones that already rendered
            for old_class in &old_classes.clone() {
                for new_class in &new_classes_clone.clone() {
                    if old_class == new_class {
                        let old_index = old_classes.iter().position(|c| c == old_class).unwrap();
                        let new_index = new_classes_clone.iter().position(|c| c ==new_class).unwrap();
                        old_classes.remove(old_index);
                        new_classes_clone.remove(new_index);
                    }
                }
            }

            //remove excess
            for old_class in &old_classes {
                el.class_list().remove_1(old_class).unwrap();
            }

            //add new class
            for new_class in &new_classes_clone {
                el.class_list().add_1(new_class).unwrap();
            }

            current_classes.set(new_classes.to_vec());
        })
    };
    Box::new(cb)
}

/* 
/// Mixin for reactive `disabled` attributes
/// 
/// ## Example
/// ```rust, no_run
/// pub fn reactive_disabled(_app: &HirolaApp) -> Dom {
///     let state = Signal::new(true);
///     let state_copy = state.clone();
///     
///     html! {
///         <div>
///             <p mixin:id=&id(&id_state) >"This is some important context"</p>
///             <button on:click=move |_| state.set(!*state.get())>"toggle submit button"</button>
///             <button mixin:disabled=&disabled(&state_copy)>"Submit"</button>
///         </div>
///     }
/// }
/// ```
pub fn disabled<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let signal = signal.clone();

    let cb = move |node: DomNode| {
        let el =node.unchecked_into::<Element>();
        let signal = signal.clone();
        create_effect(move || {
            let disabled = *signal.get();
            match disabled {
                true => el.set_attribute("disabled", "true").unwrap(),
                false => el.remove_attribute("disabled").unwrap(),
            }
            
        })
    };

    Box::new(cb)
}
*/