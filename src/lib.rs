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
    pub use hirola_macros::{component, html, mixin, FormEntity};
    
}

/// Exposing single item signal
pub mod signal {
    pub use hirola_core::prelude::signal::*;
}

/// Exposing vec signal
pub mod signal_vec {
    pub use hirola_core::prelude::signal_vec::*;
}

/// Expose the dom feature
#[cfg(feature = "dom")]
pub mod dom {
    pub use hirola_dom::*;
    pub use hirola_dom::effects::prelude::*;
}

/// Expose the ssr feature
#[cfg(feature = "ssr")]
pub mod ssr {
    pub use hirola_ssr::*;
}