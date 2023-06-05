use crate::prelude::DomNode;
use futures_signals::signal::Mutable;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MixinError {
    #[error("Invalid namespace (expected {expected:?}, got {found:?})")]
    InvalidNamespace { expected: String, found: String },
    #[error("Could not bind mixin to Node: {0:?}")]
    NodeError(DomNode),
}

pub trait Mixin {
    fn mixin(&self, namespace: &str, node: DomNode) -> Result<(), MixinError>;
}

impl<T> Mixin for T
where
    T: Fn(DomNode),
{
    fn mixin(&self, _ns: &str, node: DomNode) -> Result<(), MixinError> {
        (&self)(node);
        Ok(())
    }
}

pub trait State: Clone {
    // Get a callback that allows interacting with state
    fn callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&Self, E) + 'static,
        Self: 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            f(&state, e);
        };
        Box::new(cb)
    }
}

pub trait StateReduce<T> {
    fn mut_callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&T, E) -> T + 'static;
}

impl<T: Clone> State for Mutable<T> {}

impl<T: Clone + 'static> StateReduce<T> for Mutable<T> {
    fn mut_callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&T, E) -> T + 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            state.set(f(&state.get_cloned(), e));
        };
        Box::new(cb)
    }
}
