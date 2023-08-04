use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use web_sys::Event;

pub trait Callback<T> {
    /// Pass a callback that allows interacting with the inner value and the dom event
    /// This method returns the new value and this updates the signal.
    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(Event)>
    where
        F: Fn(&Self, Event) + 'static;
    /// Pass a callback that allows interacting with self and the dom event
    fn callback<F>(&self, f: F) -> Box<dyn Fn(Event)>
    where
        F: Fn(&Self) + 'static,
        Self: Sized;
}

impl<T: Clone + 'static> Callback<T> for Mutable<T> {
    fn callback<F>(&self, f: F) -> Box<dyn Fn(Event) + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move |_| {
            f(&state);
        };
        Box::new(cb)
    }

    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(Event) + 'static>
    where
        F: Fn(&Self, Event) + 'static,
    {
        let state = self.clone();
        let cb = move |e| {
            f(&state, e);
        };
        Box::new(cb)
    }
}

impl<T: Clone + 'static> Callback<T> for MutableVec<T> {
    fn callback<F>(&self, f: F) -> Box<dyn Fn(Event) + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move |_| {
            (f(&state));
        };
        Box::new(cb)
    }

    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(Event)>
    where
        F: Fn(&Self, Event) + 'static,
    {
        let state = self.clone();
        let cb = move |e: Event| {
            f(&state, e);
        };
        Box::new(cb)
    }
}
