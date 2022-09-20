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
/// Note: This is a security risk if the string to be inserted might contain potentially malicious content.
/// sanitize the content before it is inserted.
/// See more: https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML
pub fn rhtml<'a>(text: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_inner_html(&format!("{text}"));
    };
    Box::new(cb)
}

/// A mixin that allows adding nonsignal text
pub fn rtext<'a, D: Display>(text: &'a D) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_text_content(Some(&format!("{text}")));
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
pub fn text<T: Display>(text: &Signal<T>) -> Box<dyn Fn(DomNode) -> ()> {
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

/// Mixin that adds text to a dom node
pub fn show(shown: &Signal<bool>) -> Box<dyn Fn(DomNode) -> ()> {
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
                        "block"
                    } else {
                        "none"
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

        node.event("keyup", handler);
        input.set_value(&format!("{}", &self.0.get_untracked()));
        Ok(())
    }
}

/// Two way binding for input and signals
pub mod model {
    use super::*;
    pub fn input<T>(s: &Signal<T>) -> Model<HtmlInputElement, T> {
        Model(s.clone(), PhantomData)
    }
}
