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

use generic_node::GenericNode;
pub use hirola_macros::html;

pub mod app;
pub mod callback;
pub mod easing;
pub mod flow;
pub mod generic_node;
pub mod macros;
pub mod noderef;
pub mod reactive;
pub mod render;

#[macro_use]
pub mod styled;

#[cfg(feature = "router")]
#[cfg_attr(docsrs, doc(cfg(feature = "router")))]
pub mod router;

pub mod mixins;

pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateResult<G: GenericNode> {
    node: G,
}

impl<G: GenericNode> TemplateResult<G> {
    /// Create a new [`TemplateResult`] from a [`GenericNode`].
    pub fn new(node: G) -> Self {
        Self { node }
    }

    /// Create a new [`TemplateResult`] with a blank comment node
    pub fn empty() -> Self {
        Self::new(G::marker())
    }

    pub fn inner_element(&self) -> G {
        self.node.clone()
    }
}

/// Render a [`TemplateResult`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render(template_result: impl FnOnce() -> TemplateResult<generic_node::DomNode>) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(template_result, &document.body().unwrap());
}

/// Render a [`TemplateResult`] under a `parent` node.
/// For rendering under the `<body>` tag, use [`render()`] instead.
///
/// _This API requires the following crate features to be activated: `dom`_
#[cfg(feature = "dom")]
pub fn render_to(
    template_result: impl FnOnce() -> TemplateResult<generic_node::DomNode>,
    parent: &web_sys::Node,
) {
    let owner = reactive::create_root(|| {
        parent
            .append_child(&template_result().node.inner_element())
            .unwrap();
    });

    thread_local! {
        static GLOBAL_OWNERS: std::cell::RefCell<Vec<reactive::Owner>> = std::cell::RefCell::new(Vec::new());
    }

    GLOBAL_OWNERS.with(|global_owners| global_owners.borrow_mut().push(owner));
}

/// Render a [`TemplateResult`] into a static [`String`]. Useful for rendering to a string on the server side.
///
/// _This API requires the following crate features to be activated: `ssr`_
#[cfg(feature = "ssr")]
pub fn render_to_string(
    template_result: impl FnOnce() -> TemplateResult<generic_node::SsrNode>,
) -> String {
    let mut ret = None;
    let _owner =
        reactive::create_root(|| ret = Some(format!("{}", template_result().inner_element())));

    ret.unwrap()
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]

pub type AsyncResult<T> = prelude::Signal<Option<Result<T, wasm_bindgen::JsValue>>>;

/// Helper for making async calls
/// 
/// _This API requires the following crate features to be activated: `dom`_
/// 
/// # Example
/// ```rust, no_run
/// fn your_page(_app: &HirolaApp) -> Dom {
///    let response = Signal::new(String::new());
///    
///    let submit_callback = response.callback(move |st, _e: Event| {
///        let state = st.clone();
/// 
///        let async_task = async {
///             //Some blocking task here...
///             String::from("Async process done")
///        }; 
///    
///        let async_response: Signal<Option<String>> = use_async(async_task);
/// 
///        create_effect(move || {
///            match async_response.get().as_ref().clone() {
///                Some(resp) => state.set(resp),
///                None => ()
///            }
///        });
///    });
///
///
///    html!{
///        <div>
///            <button on:click=submit_callback >"Submit"</button>
///        </div>
///    }
/// }
/// ```
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub fn use_async<F, T: 'static>(future: F) -> prelude::Signal<Option<T>>
where
    F: std::future::Future<Output = T> + 'static,
{
    let handler = prelude::Signal::new(None);
    let inner = handler.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let res = future.await;
        inner.set(Some(res));
    });
    handler
}

/// The maple prelude.
pub mod prelude {
    pub use hirola_macros::{component, html};

    pub use crate::cloned;
    pub use crate::flow::{Indexed, IndexedProps, Keyed, KeyedProps};
    #[cfg(feature = "dom")]
    pub use crate::generic_node::DomNode;
    pub use crate::generic_node::GenericNode;
    #[cfg(feature = "ssr")]
    pub use crate::generic_node::SsrNode;
    pub use crate::noderef::NodeRef;
    pub use crate::reactive::{
        create_effect, create_effect_initial, create_memo, create_root, create_selector,
        create_selector_with, on_cleanup, untrack, Signal, StateHandle,
    };
    pub use crate::render::Render;
    #[cfg(feature = "ssr")]
    pub use crate::render_to_string;
    pub use crate::TemplateResult;
    #[cfg(feature = "dom")]
    pub use crate::{render, render_to};

    pub use crate::callback::Mixin;
    pub use crate::callback::State;
    pub use crate::callback::StateReduce;

    pub use crate::app::*;
    #[cfg(feature = "router")]
    pub use crate::router::*;
    #[cfg(feature = "async")]
    pub use crate::use_async;
    #[cfg(feature = "async")]
    pub use crate::AsyncResult;

    pub use crate::styled::*;

    pub use crate::style;

    pub use crate::mixins;
}
