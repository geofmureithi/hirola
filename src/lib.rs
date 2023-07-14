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
//! ```rust,no_run
//! use hirola::prelude::*;
//! use hirola::signal::Mutable;
//!
//! fn counter(_: &App<()>) -> ViewBuilder {
//!     let state = Mutable::new(99);
//!     let decrement = state.update_with(|count, _| count.set(count.get() - 1));
//!     let increment = state.update_with(|count, _| count.set(count.get() - 1));
//!
//!     html! {
//!         <div class="flex flex-row h-10">
//!             <button on:click=decrement>"-"</button>
//!             <span>{state}</span>
//!             <button on:click=increment>"+"</button>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     let window = web_sys::window().unwrap();
//!     let document = window.document().unwrap();
//!     let body = document.body().unwrap();
//!
//!     let mut app = App::new(());
//!     app.route("/", counter);
//!     app.mount(&body);
//! }
//! ```
//!
//!
//! ## Features
#![cfg_attr(
    feature = "docsrs",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

/// The defaults from core
pub mod prelude {
    pub use hirola_core::prelude::*;
}

/// Exposing single item signal
pub mod signal {
    pub use hirola_core::prelude::signal::*;
}

/// Exposing vec signal
pub mod signal_vec {
    pub use hirola_core::prelude::signal_vec::*;
}

/// Include form mixins and utilities
#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub mod form {
    pub use hirola_form::*;
}
