//! # Hirola API Documentation
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

use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
use std::{future::Future, pin::Pin};
use crate::dom::*;
pub use hirola_macros::html;

pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

#[cfg(feature = "app")]
pub mod app;
pub mod dom;
pub mod effect;
pub mod generic_node;
pub mod mixins;
pub mod render;
pub mod templating;
pub mod update;

#[cfg(feature = "dom")]
use crate::generic_node::DomNode;

/// Render a [`Dom`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render(root: Dom) -> Result<Dom, render::Error> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(root, &document.body().unwrap())
}

/// Render a [`Dom`] under a `parent` node.
/// For rendering under the `<body>` tag, use [`render()`] instead.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render_to(dom: dom::Dom, parent: &web_sys::Node) -> Result<dom::Dom, render::Error> {
    dom.mount(&DomNode {
        node: parent.clone(),
    })
}

/// Render a [`Dom`] into a static [`String`]. Useful for rendering to a string on the server side.
///
/// _This API requires the following crate features to be activated: `ssr`_
#[cfg(feature = "ssr")]
pub fn render_to_string(root: Dom) -> String {
    use crate::generic_node::GenericNode;
    use crate::generic_node::SsrNode;
    use crate::render::Render;
    let node = SsrNode::fragment();
    let dom = Dom::new_from_node(&node);
    Render::render_into(Box::new(root), &dom).unwrap();
    format!("{}", dom.node())
}

#[inline]
pub fn spawn<F>(future: F) -> DiscardOnDrop<CancelableFutureHandle>
where
    F: Future<Output = ()> + 'static,
{
    let (handle, future) = cancelable_future(future, || ());

    #[cfg(feature = "dom")]
    wasm_bindgen_futures::spawn_local(future);

    #[cfg(not(feature = "dom"))]
    drop(future);
    // tokio::task::spawn_local(future);

    handle
}

pub mod prelude {

    // pub use crate::spawn;
    pub use crate::effect::SideEffect;
    #[cfg(feature = "dom")]
    pub use crate::generic_node::DomNode as DomType;
    pub use crate::generic_node::GenericNode;
    #[cfg(feature = "ssr")]
    pub use crate::generic_node::SsrNode as DomType;
    pub use crate::templating::flow::{Indexed, IndexedProps};
    pub use crate::templating::noderef::NodeRef;
    pub use crate::templating::suspense::{Suspend, Suspense, SuspenseResult::*};
    pub use crate::templating::switch::Switch;
    pub use futures_signals::*;
    pub use hirola_macros::{component, html};

    #[cfg(feature = "ssr")]
    pub use crate::render_to_string;
    #[cfg(feature = "dom")]
    pub use crate::{render, render_to};

    pub use crate::dom::Dom;
    pub use crate::update::Update;

    #[cfg(feature = "app")]
    pub use crate::app::*;
    pub use crate::render::*;
    pub use crate::BoxedLocal;

    pub use crate::mixins::*;
}
