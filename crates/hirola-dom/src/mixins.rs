use std::fmt::Display;

use hirola_core::{prelude::signal::{Signal, SignalExt}, generic_node::GenericNode};
use wasm_bindgen::JsCast;
use web_sys::Element;

use crate::Dom;

/// A mixin that allows adding raw html
/// Note: This is a security risk if the string to be inserted might contain potentially malicious content.
/// sanitize the content before it is inserted.
/// See more: https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML
#[allow(unused_variables)]
pub fn raw_html<'a>(text: &'a str) -> Box<dyn Fn(&Dom) -> () + 'a> {
    let cb = move |node: &Dom| {
        let element = node.as_ref().clone().unchecked_into::<Element>();
        element.set_inner_html(text);
    };
    Box::new(cb)
}

/// A mixin that allows adding non-signal text
#[allow(unused_variables)]
pub fn raw_text<'a>(text: &'a str) -> Box<dyn Fn(&Dom) + 'a> {
    let cb = move |dom: &Dom| {
        dom.node.set_text_content(Some(&format!("{text}")));
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
#[allow(unused_variables)]
pub fn text<T, S>(text: &S) -> Box<dyn Fn(&Dom)>
where
    T: Display + 'static,
    S: Signal<Item = T> + SignalExt + Clone + 'static,
{
    let signal = text.clone();

    let cb = move |_node: &Dom| {
        use std::future::ready;
        let element = _node.as_ref().clone().unchecked_into::<Element>();
        let signal = signal.clone();
        let future = signal.for_each(move |value| {
            element.set_text_content(Some(&format!("{}", value)));
            ready(())
        });
        _node.effect(future);
    };
    Box::new(cb)
}
