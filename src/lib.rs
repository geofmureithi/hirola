#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Hirola API Documentation
//! **Hirola** is an un-opinionated and extensible web framework for that is focused on simplicity and predictability.
//!
//! ## Example
//! ```ignore
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
//!     let window = web_sys::window().unwrap();
//!     let document = window.document().unwrap();
//!     let body = document.body().unwrap();
//!
//!     let app = HirolaApp::new();
//!     app.mount(&body, counter);
//! }
//! ```
//!
//!
//! ## Features
#![cfg_attr(
    feature = "docsrs",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

//! Hirola is derived from a fork of [maple reactivity core](https://github.com/lukechu10/maple).
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
