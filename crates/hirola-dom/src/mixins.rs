use std::fmt::Display;

use crate::effects::attr_mixin::XEffect;
use crate::Dom;
use hirola_core::effect::SideEffect;
use hirola_core::prelude::signal::{DedupeCloned, DedupeMap};
use hirola_core::prelude::EffectAttribute;
use hirola_core::{
    generic_node::GenericNode,
    prelude::signal::{Signal, SignalExt},
};
use hirola_macros::mixin;
use wasm_bindgen::JsCast;
use web_sys::Element;

/// A mixin that allows adding non-signal text
#[allow(unused_variables)]
#[mixin]
pub fn raw_text<'a>(text: &'a str) -> Box<dyn Fn(&Dom) + 'a> {
    let cb = move |dom: &Dom| {
        dom.node.set_text_content(Some(text));
    };
    Box::new(cb)
}

pub struct Html;

fn html<'a>(text: &'a str) -> Box<dyn Fn(&Dom) + 'a> {
    let cb = move |node: &Dom| {
        let dom = node.inner_element();
        let element = dom.dyn_ref::<Element>().unwrap();
        element.set_inner_html(text); // Remember to escape this.
    };
    Box::new(cb)
}

impl EffectAttribute for Html {
    type Handler = XEffect;
    fn read_as_attr(&self) -> String {
        "html".to_owned()
    }
}

impl SideEffect<Html, &str, Dom> for XEffect {
    fn effect(&self, node: &Dom, _: Html, text: &str) {
        html(text)(node)
    }
}

pub struct Text;

impl EffectAttribute for Text {
    type Handler = XEffect;
    fn read_as_attr(&self) -> String {
        "text".to_owned()
    }
}

impl<
        F: FnMut(&mut <S as Signal>::Item) -> A + 'static,
        S: Signal + 'static,
        A: Display + 'static + Clone + PartialEq,
    > SideEffect<Text, DedupeMap<S, F>, Dom> for XEffect
where
    <S as Signal>::Item: PartialEq,
{
    fn effect(&self, node: &Dom, _attr: Text, effect: DedupeMap<S, F>) {
        let dom = node.clone();
        let element = dom.dyn_into::<Element>().unwrap();
        let future = SignalExt::dedupe_map(effect, move |value| {
            element.set_text_content(Some(&format!("{}", value)));
        })
        .to_future();
        node.effect(future);
    }
}

impl<S: Signal + 'static> SideEffect<Text, DedupeCloned<S>, Dom> for XEffect
where
    <S as Signal>::Item: PartialEq + Display + Clone,
{
    fn effect(&self, node: &Dom, _attr: Text, effect: DedupeCloned<S>) {
        let dom = node.clone();
        let element = dom.dyn_into::<Element>().unwrap();
        let future = SignalExt::dedupe_map(effect, move |value| {
            element.set_text_content(Some(&format!("{}", value)));
        })
        .to_future();
        node.effect(future);
    }
}
