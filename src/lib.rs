#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Hirola API Documentation
//! **Hirola** is an opinionated and extensible web framework for that is focused on simplicity and predictability.
//!
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//!
//! fn counter(_: &HirolaApp) -> Dom {
//!     let state = Signal::new(99);
//!     let decerement = state.mut_callback(|count, _| *count - 1);
//!     let incerement = state.mut_callback(|count, _| *count + 1);
//!
//!     html! {
//!         <div class="flex flex-row h-10">
//!             <button on:click=decerement>"-"</button>
//!             <input value=state.get() disabled/>
//!             <button on:click=incerement>"+"</button>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     let app = HirolaApp::new();
//!     app.mount("body", counter);
//! }
//! ```
//!
//!
//! ## Features
#![cfg_attr(
    feature = "docsrs",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]
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
//!    let app = HirolaApp::new();
//!    app.mount("body", mixin_demo);
//! }
//! ```

//! Hirola is derived from a fork of [marple reactivity core](https://github.com/lukechu10/maple).

/// The defaults from core
pub mod prelude {
    pub use super::*;
    pub use hirola_core::prelude::*;
}

/// Include form mixins and utilities
#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub mod form {
    pub use hirola_form::*;
}
