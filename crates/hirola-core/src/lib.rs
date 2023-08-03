//! # hirola-core
//!
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//! use hirola::signal::Mutable;
//!
//! fn counter() -> Dom {
//!     let count = Mutable::new(0i32);
//!     let decrement = count.callback(|s| *s.lock_mut() -= 1);
//!     let increment = count.callback(|s| *s.lock_mut() += 1);
//!     html! {
//!          <>
//!             <button on:click=decrement>"-"</button>
//!             <span>{count}</span>
//!             <button on:click=increment>"+"</button>
//!          </>
//!     }
//! }
//!
//! fn main() {
//!     let root = render(counter()).unwrap();
//!     std::mem::forget(root);
//! }
//! ```
//! ## Features
//! - `dom` (_default_) - Enables rendering templates to DOM nodes. Only useful on `wasm32-unknown-unknown` target.
//! - `ssr` - Enables rendering templates to static strings (useful for Server Side Rendering / Server side Generation).
//! - `serde` - Enables serializing and deserializing `Signal`s and other wrapper types using `serde`.

#![allow(non_snake_case)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::rc_buffer)]
#![deny(clippy::trait_duplication_in_bounds)]
#![deny(clippy::type_repetition_in_bounds)]
use crate::dom::*;
use discard::DiscardOnDrop;
use futures_signals::{cancelable_future, CancelableFutureHandle};
pub use hirola_macros::html;
use std::{future::Future, pin::Pin};

pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "dom")]
pub mod callback;
pub mod dom;
pub mod effect;
pub mod generic_node;
#[cfg(feature = "dom")]
pub mod mixins;
pub mod render;
pub mod templating;

#[cfg(feature = "dom")]
use crate::generic_node::DomNode;

/// Render a [`Dom`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render(dom: Dom) -> Result<Dom, render::Error> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(dom, &document.body().unwrap())
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
pub fn render_to_string(dom: Dom) -> String {
    use crate::generic_node::GenericNode;
    use crate::generic_node::SsrNode;
    use crate::render::Render;
    let node = SsrNode::fragment();
    let root = Dom::new_from_node(&node);
    Render::render_into(Box::new(dom), &root).unwrap();
    format!("{}", root.node())
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
    #[cfg(feature = "dom")]
    pub use crate::callback::Callback;
    pub use crate::dom::Dom;

    #[cfg(feature = "app")]
    pub use crate::app::*;
    pub use crate::render::*;
    pub use crate::BoxedLocal;
    #[cfg(feature = "dom")]
    pub use crate::mixins::*;

    pub use futures_signals::signal::Mutable;
    pub use futures_signals::signal_vec::MutableVec;
    pub use futures_signals::signal_map::MutableBTreeMap;
}
