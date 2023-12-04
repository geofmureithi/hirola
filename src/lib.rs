#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]
//! ## Features
#![cfg_attr(
    feature = "docsrs",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

/// The defaults imports
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

/// App introduces state and routing management.
/// Use if you need to store data shared between routes or just routing.
#[cfg(feature = "app")]
pub mod app {
    // pub use hirola_core::app::*;
}

/// Include form mixins and utilities
#[cfg(feature = "form")]
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
pub mod form {
    pub use hirola_form::*;
}
