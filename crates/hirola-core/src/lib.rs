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
pub use hirola_macros::html;
use std::{future::Future, pin::Pin};

pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// #[cfg(feature = "app")]
// pub mod app;
pub mod effect;
pub mod generic_node;
// pub mod mixins;
pub mod render;
pub mod templating;
pub mod callback;

pub mod prelude {

    // pub use crate::spawn;
    pub use crate::effect::SideEffect;
    pub use crate::generic_node::GenericNode;
    pub use crate::generic_node::EventListener;
    pub use crate::templating::flow::{Indexed, IndexedProps};
    pub use crate::templating::suspense::{Suspend, Suspense, SuspenseResult::*};
    pub use crate::templating::switch::Switch;
    pub use futures_signals::*;
    pub use hirola_macros::{component, html};

    // #[cfg(feature = "app")]
    // pub use crate::app::*;

    // pub use crate::mixins::*;
    pub use crate::render::*;
    pub use crate::BoxedLocal;

    pub use crate::generic_node::*;
    pub use crate::callback::Callback;

    pub use futures_signals::signal::Mutable;
    pub use futures_signals::signal_map::MutableBTreeMap;
    pub use futures_signals::signal_vec::MutableVec;
}

#[cfg(feature = "dom")]
pub mod dom_test_utils {
    use wasm_bindgen::{prelude::Closure, JsCast};

    pub fn next_tick_with<N: Clone + 'static>(with: &N, f: impl Fn(&N) -> () + 'static) {
        let with = with.clone();
        let f: Box<dyn Fn() -> ()> = Box::new(move || f(&with));
        let a = Closure::<dyn Fn()>::new(f);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(a.as_ref().unchecked_ref())
            .unwrap();
    }

    pub fn next_tick<F: Fn() + 'static>(f: F) {
        let a = Closure::<dyn Fn()>::new(move || f());
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(a.as_ref().unchecked_ref())
            .unwrap();
    }
}
