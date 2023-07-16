#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # Hirola API Documentation
//! **Hirola** is an un-opinionated and extensible web framework for that is focused on simplicity and predictability.
//! Hirola uses frp-signals under the hood and has no vdom meaning fine-tuned reactivity.
//! 
//!
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//! use hirola::signal::Mutable;
//! 
//! fn counter(app: &App<Mutable<i32>>) -> Dom {
//!     let state = app.state();
//!     let decrement = |_| state.replace_with(|c| c--);
//!     let increment = |_| state.replace_with(|c| c++); 
//!     html! {
//!         <>
//!            <button on:click=decrement>"-"</button>
//!            <span>{state}</span>
//!            <button on:click=increment>"+"</button>
//!         </>
//!    }
//! }
//!
//! fn main() {
//!    let state = Mutable::new(99);
//!    let mut app = App::new(state);
//!    app.route("/", counter);
//!    app.mount();
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
