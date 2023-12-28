//! # hirola-core
//!
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//! use hirola::signal::Mutable;
//! use hirola::dom::*;
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
//! ```

#![allow(non_snake_case)]
#![warn(clippy::clone_on_ref_ptr)]
#![warn(clippy::rc_buffer)]
#![deny(clippy::trait_duplication_in_bounds)]
#![deny(clippy::type_repetition_in_bounds)]
use std::{future::Future, pin::Pin};
pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

pub mod callback;
pub mod effect;
pub mod generic_node;
pub mod render;
pub mod templating;

pub mod prelude {
    pub use crate::callback::Callback;
    pub use crate::effect::*;
    pub use crate::generic_node::EventListener;
    pub use crate::generic_node::GenericNode;
    pub use crate::generic_node::*;
    pub use crate::render::*;
    pub use crate::templating::flow::{Indexed, IndexedProps};
    pub use crate::templating::suspense::{Suspend, Suspense, SuspenseResult::*};
    pub use crate::templating::switch::Switch;
    pub use crate::BoxedLocal;
    pub use futures_signals::signal::Mutable;
    pub use futures_signals::signal_map::MutableBTreeMap;
    pub use futures_signals::signal_vec::MutableVec;
    pub use futures_signals::*;
}
