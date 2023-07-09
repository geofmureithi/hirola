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

use std::{future::Future, pin::Pin};

use builder::ViewBuilder;
use generic_node::{DomNode, GenericNode};
pub use hirola_macros::html;

pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub mod app;

pub mod generic_node;

pub mod render;
pub mod update;

#[cfg(feature = "router")]
#[cfg_attr(docsrs, doc(cfg(feature = "router")))]
pub mod router;

// pub mod mixins;

mod builder;
mod effect;
mod templating;
mod view;

/// Render a [`ViewBuilder`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render(builder: ViewBuilder<DomNode>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(builder, &document.body().unwrap());
}

/// Render a [`TemplateResult`] under a `parent` node.
/// For rendering under the `<body>` tag, use [`render()`] instead.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render_to(builder: ViewBuilder<DomNode>, parent: &web_sys::Node) {
    let view = builder
        .mount(&DomNode {
            node: parent.clone(),
        })
        .unwrap();
    std::mem::forget(view);
}

/// Render a [`TemplateResult`] into a static [`String`]. Useful for rendering to a string on the server side.
///
/// _This API requires the following crate features to be activated: `ssr`_
#[cfg(feature = "ssr")]
pub fn render_to_string(builder: ViewBuilder<generic_node::SsrNode>) -> String {
    use crate::generic_node::SsrNode;
    use crate::render::Render;
    use crate::view::View;
    let node = SsrNode::fragment();
    let view = View::new_from_node(&node);
    Render::render_into(Box::new(builder), &view);
    format!("{}", view.node())
}

pub mod prelude {
    // pub use crate::spawn;
    pub use crate::templating::switch::Switch;
    pub use crate::templating::suspense::Suspense;
    pub use futures_signals::signal::*;
    pub use futures_signals::signal_vec::*;
    pub use hirola_macros::{component, html};

    pub use crate::effect::SideEffect;
    #[cfg(feature = "dom")]
    pub use crate::generic_node::DomNode;
    pub use crate::generic_node::GenericNode;
    #[cfg(feature = "ssr")]
    pub use crate::generic_node::SsrNode;
    pub use crate::templating::flow::{Indexed, IndexedProps};
    pub use crate::templating::noderef::NodeRef;

    pub use crate::render::Render;
    pub use crate::render::RenderMap;
    #[cfg(feature = "ssr")]
    pub use crate::render_to_string;
    pub use crate::update::Identity;
    #[cfg(feature = "dom")]
    pub use crate::{render, render_to};

    pub use crate::update::Mixin;

    pub use crate::update::Update;

    pub use crate::view::View;
    pub use crate::builder::ViewBuilder;


    pub use crate::builder::html::HtmlBuilder;

    pub use crate::BoxedLocal;

    #[cfg(feature = "router")]
    pub use crate::app::*;
    
    #[cfg(feature = "router")]
    pub use crate::router::*;

    // pub use crate::use_async;

    // pub use crate::AsyncResult;

    // pub use crate::styled::*;

    // pub use crate::style;

    // pub use crate::mixins;
}
