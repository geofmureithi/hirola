//! ## Mixins
//! Hirola aims to be extensible and follow DRY principles.
//! Here is an example of a mixin
//! ```rust,no_run
//! use hirola::prelude::*;
//! use web_sys::Element;
//! // Mixin that controls tailwind opacity based on a bool signal
//! fn opacity<'a>(signal: &'a Mutable<bool>) -> Box<dyn Fn(DomType) -> () + 'a> {
//!    let cb = move |node: DomType| {
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
//! fn mixin_demo(_app: &App<S, G>) -> Dom {
//!    let is_shown = Mutable::new(true);
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
//!     let app = App<S, G>::new();
//!
//!     app.mount(&body, mixin_demo);
//! }
//! ```
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use futures_signals::signal::{Mutable, MutableSignalRef, SignalExt};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{Element, Event, HtmlElement, HtmlInputElement};

use crate::{generic_node::GenericNode, update::Identity, view::View};

/// A mixin that allows adding raw html
/// Note: This is a security risk if the string to be inserted might contain potentially malicious content.
/// sanitize the content before it is inserted.
/// See more: https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML
pub fn rhtml<'a, G: GenericNode>(text: &'a str) -> Box<dyn Fn(&View) -> () + 'a> {
    let cb = move |node: &View| {
        let element = node
            .inner_element()
            .as_ref()
            .clone()
            .unchecked_into::<Element>();
        element.set_inner_html(text);
    };
    Box::new(cb)
}

/// A mixin that allows adding nonsignal text
pub fn rtext<'a, D: Display, G: GenericNode>(text: &'a D) -> Box<dyn Fn(&View) + 'a> {
    let cb = move |view: &View| {
        view.node().set_text_content(Some(&format!("{text}")));
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
pub fn text<T: Display + Clone + 'static>(text: &Mutable<T>) -> Box<dyn Fn(&Dom)> {
    let signal = text.clone();
    let cb = move |node: &Dom| {
        let element = node
            .inner_element()
            .as_ref()
            .clone()
            .unchecked_into::<Element>();
        let fut = create_effect_lite(&signal, move |value| {
            let element = element.clone();
            element.set_text_content(Some(&format!("{}", value)));
        })
        .to_future();
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
pub fn show(shown: &Mutable<bool>) -> Box<dyn Fn(&Dom)> {
    let signal = shown.clone();
    let cb = move |node: &Dom| {
        let element = node
            .inner_element()
            .as_ref()
            .clone()
            .unchecked_into::<HtmlElement>();
        let fut = create_effect_lite(&signal, move |value| {
            let element = element.clone();
            let style = element.style();
            style
                .set_property("display", {
                    if *value {
                        "block"
                    } else {
                        "none"
                    }
                })
                .unwrap();
        })
        .to_future();
        node.effect(fut);
    };
    Box::new(cb)
}

/// Mount an effect to a dom node, ensuring its bound
pub fn mount_effect<T: 'static, F: 'static>(
    signal_ref: MutableSignalRef<T, F>,
) -> Box<dyn Fn(DomType)>
where
    F: FnMut(&T) -> (),
{
    spawn_local(signal_ref.to_future());
    let cb = move |_: DomType| {};
    Box::new(cb)
}

/// Model allows 2-way binding eg between a signal and an input
pub struct Model<Node, T: 'static>(Mutable<T>, PhantomData<Node>);

impl<T: Display + FromStr + Clone + 'static> Mixin<Identity> for Model<HtmlInputElement, T>
where
    <T as FromStr>::Err: Debug,
{
    fn mixin(&self, node: &Dom) {
        let input = {
            let node = node.inner_element().as_ref().clone();
            node.dyn_into::<HtmlInputElement>().unwrap()
        };
        let signal = self.0.clone();
        create_effect(signal.clone(), move |value| {
            input.set_value(&format!("{}", value));
        });
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
    }
}

/// Two way binding for input and signals
pub mod model {
    use super::*;
    /// Bind a [HtmlInputElement] to a [Mutable<T>]
    pub fn input<T>(s: &Mutable<T>) -> Model<HtmlInputElement, T> {
        Model(s.clone(), PhantomData)
    }
}
