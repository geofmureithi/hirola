//! # Hirola API Documentation
//! **Hirola** is an opinionated web framework for that is focused on simplicity and predictability.
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//!
//! fn counter(_: &HirolaApp) -> Dom {
//!    let state = Signal::new(99);
//!     let decerement = state.mut_callback(|count, _| *count - 1);
//!     let incerement = state.mut_callback(|count, _| *count + 1);
//!
//!     html! {
//!         <div class="flex flex-row h-10">
//!             <button on:click={decerement}>"-"</button>
//!             <input value={state.get()} disabled/>
//!             <button on:click={incerement}>"+"</button>
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
//! - `dom` (_default_) - Enables rendering templates to DOM nodes. Only useful on `wasm32-unknown-unknown` target.
//! - `ssr` - Enables rendering templates to static strings (useful for Server Side Rendering / Pre-rendering).
//! - `serde` - Enables serializing and deserializing `Signal`s and other wrapper types using `serde`.
//! - `form` - Enables form management mixins
//!

// Hirola is based on [marple reactivity core](https://github.com/lukechu10/maple).
extern crate hirola_core;

pub mod prelude {
    pub use super::*;
    pub use hirola_core::prelude::*;
    pub use web_sys::Event;
}

/// Include Form mixins
#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub mod form {
    pub use hirola_form::*;
}
