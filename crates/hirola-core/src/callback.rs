use crate::prelude::DomNode;

pub trait StateReduce<T> {
    /// Sets the return value.
    /// 
    /// # Example
    /// ```
    /// use hirola::prelude::{*, State};
    /// use web_sys::Event;
    /// 
    /// pub struct PageState{
    ///     pub counter: Signal<u32>
    /// }
    /// 
    /// impl PageState {
    ///     fn add_one(&self) -> Box<dyn Fn(Event)> {
    ///         self.counter.mut_callback(move |counter: &u32, _e: Event| {
    ///             *counter + 1
    ///         })
    ///     }
    /// }
    /// 
    /// pub fn your_page(_app: HirolaApp) -> Dom {
    ///     let state = PageState{counter: Signal::new(0)};
    ///     let add_one = state.add_one();
    ///     
    ///     html! {
    ///         <div>
    ///             <p>"counter is: "{state.counter.get()}</p>
    ///             <button on:click=add_one>"add one"</button>
    ///         </div>
    ///     }
    /// }
    /// ```
    fn mut_callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&T, E) -> T + 'static;
}

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
    /// Get a callback that allows interacting with state
    /// 
    /// # Example
    /// ```
    /// use hirola::prelude::*;
    /// use web_sys::Event; 
    /// 
    /// pub struct PageState{
    ///     pub counter: Signal<u32>
    /// }
    /// 
    /// impl PageState {
    ///     fn add_one(&self) -> Box<dyn Fn(Event)> {
    ///         self.counter.callback(move |counter: &Signal<u32>, _e: Event| {
    ///             let num: u32 = *counter.get();
    ///             counter.set(num + 1);
    ///         })
    ///     }
    /// }
    /// 
    /// pub fn your_page(_app: HirolaApp) -> Dom {
    ///     let state = PageState{counter: Signal::new(0)};
    ///     let add_one = state.add_one();
    ///     
    ///     html! {
    ///         <div>
    ///             <p>"counter is: "{state.counter.get()}</p>
    ///             <button on:click=add_one>"add one"</button>
    ///         </div>
    ///     }
    /// }
    /// ```
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
