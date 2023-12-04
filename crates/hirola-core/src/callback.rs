use futures_signals::{signal::Mutable, signal_vec::MutableVec};


pub trait Callback<T, E = ()> {
    /// Pass a callback that allows interacting with the inner value and the dom event
    /// This method returns the new value and this updates the signal.
    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&Self, E) + 'static;
    /// Pass a callback that allows interacting with self and the dom event
    fn callback<F>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&Self) + 'static,
        Self: Sized;
}

impl<T: Clone + 'static, E> Callback<T, E> for Mutable<T> {
    fn callback<F>(&self, f: F) -> Box<dyn Fn(E) + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move |_| {
            f(&state);
        };
        Box::new(cb)
    }

    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(E) + 'static>
    where
        F: Fn(&Self, E) + 'static,
    {
        let state = self.clone();
        let cb = move |e| {
            f(&state, e);
        };
        Box::new(cb)
    }
}

impl<T: Clone + 'static, E> Callback<T, E> for MutableVec<T> {
    fn callback<F>(&self, f: F) -> Box<dyn Fn(E) + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move |_| {
            (f(&state));
        };
        Box::new(cb)
    }

    fn callback_with<F>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&Self, E) + 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            f(&state, e);
        };
        Box::new(cb)
    }
}
