//! # Hirola API Documentation
//!
//! Hirola is based on [Marple](https://github.com/lukechu10/maple).
//!
//! ## Features
//! - `dom` (_default_) - Enables rendering templates to DOM nodes. Only useful on `wasm32-unknown-unknown` target.
//! - `ssr` - Enables rendering templates to static strings (useful for Server Side Rendering / Pre-rendering).
//! - `serde` - Enables serializing and deserializing `Signal`s and other wrapper types using `serde`.

#![allow(non_snake_case)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::rc_buffer)]
#![deny(clippy::trait_duplication_in_bounds)]
#![deny(clippy::type_repetition_in_bounds)]

use futures_signals::signal::{Mutable, SignalExt};
use generic_node::GenericNode;
pub use hirola_macros::html;

pub mod app;
pub mod callback;
pub mod easing;
pub mod generic_node;
pub mod macros;
pub mod noderef;
pub mod render;

#[macro_use]
pub mod styled;

#[cfg(feature = "router")]
#[cfg_attr(docsrs, doc(cfg(feature = "router")))]
pub mod router;

pub mod mixins;

pub mod utils;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum TemplateError {
    #[error("Error occurred in Node")]
    NodeError,
}

pub type TemplateResult<G> = Result<G, TemplateError>;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct TemplateResult<G: GenericNode> {
//     node: G,
// }

// impl<G: GenericNode> TemplateResult<G> {
//     /// Create a new [`TemplateResult`] from a [`GenericNode`].
//     pub fn new(node: G) -> Self {
//         Self { node }
//     }

//     /// Create a new [`TemplateResult`] with a blank comment node
//     pub fn empty() -> Self {
//         Self::new(G::marker())
//     }

//     pub fn inner_element(&self) -> G {
//         self.node.clone()
//     }
// }

/// Render a [`TemplateResult`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render(template_result: impl FnOnce() -> TemplateResult<generic_node::DomNode>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(|| template_result().unwrap(), &document.body().unwrap());
}

/// Render a [`TemplateResult`] under a `parent` node.
/// For rendering under the `<body>` tag, use [`render()`] instead.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render_to(template_result: impl FnOnce() -> generic_node::DomNode, parent: &web_sys::Node) {
    parent
        .append_child(&template_result().inner_element())
        .unwrap();
}

/// Render a [`TemplateResult`] into a static [`String`]. Useful for rendering to a string on the server side.
///
/// _This API requires the following crate features to be activated: `ssr`_
#[cfg(feature = "ssr")]
pub fn render_to_string(
    template_result: impl FnOnce() -> TemplateResult<generic_node::SsrNode>,
) -> String {
    format!("{}", template_result().unwrap().inner_element())
}

pub type AsyncResult<T> = Mutable<Option<Result<T, wasm_bindgen::JsValue>>>;

pub fn use_async<F, T: 'static>(future: F) -> Mutable<Option<T>>
where
    F: std::future::Future<Output = T> + 'static,
{
    let handler = Mutable::new(None);
    let inner = handler.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let res = future.await;
        inner.set(Some(res));
    });
    handler
}

pub fn create_effect<A: Clone + 'static>(reactive: Mutable<A>, mut cb: impl FnMut(A) + 'static) {
    let fut = reactive.signal_cloned().for_each(move |val| {
        cb(val);
        async {}
    });
    wasm_bindgen_futures::spawn_local(fut);
}

/// The maple prelude.
pub mod prelude {
    pub use futures_signals::signal::Mutable as Signal;
    pub use futures_signals::signal::ReadOnlyMutable as StateHandle;
    pub use super::create_effect;
    pub use hirola_macros::{component, html};

    pub use crate::cloned;
    // pub use crate::flow::{Indexed, IndexedProps, Keyed, KeyedProps};
    #[cfg(feature = "dom")]
    pub use crate::generic_node::DomNode;
    pub use crate::generic_node::GenericNode;
    #[cfg(feature = "ssr")]
    pub use crate::generic_node::SsrNode;
    pub use crate::noderef::NodeRef;
    // pub use crate::reactive::{
    //     create_effect, create_effect_initial, create_memo, create_root, create_selector,
    //     create_selector_with, on_cleanup, untrack, Signal, StateHandle,
    // };
    pub use crate::render::Render;
    #[cfg(feature = "ssr")]
    pub use crate::render_to_string;
    pub use crate::TemplateError;
    pub use crate::TemplateResult;
    #[cfg(feature = "dom")]
    pub use crate::{render, render_to};

    pub use crate::callback::Mixin;
    pub use crate::callback::State;
    pub use crate::callback::StateReduce;

    pub use crate::app::*;
    #[cfg(feature = "router")]
    pub use crate::router::*;

    pub use crate::use_async;

    pub use crate::AsyncResult;

    pub use crate::styled::*;

    pub use crate::style;

    pub use crate::mixins;
}
