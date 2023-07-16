use futures_signals::{signal::Mutable, signal_vec::MutableVec};

pub trait Update<T> {
    /// Pass a callback that allows interacting with the inner value and the dom event
    /// This method returns the new value and this updates the signal.
    fn update_with<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&Self, E) + 'static;
    /// Pass a callback that allows interacting with self and the dom event
    fn update<F>(&self, f: F) -> Box<dyn Fn()>
    where
        F: Fn(&Self) + 'static,
        Self: Sized;
}

impl<T: Clone + 'static> Update<T> for Mutable<T> {
    fn update<F>(&self, f: F) -> Box<dyn Fn() + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move || {
            f(&state);
        };
        Box::new(cb)
    }

    fn update_with<F, E>(&self, f: F) -> Box<dyn Fn(E) + 'static>
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

impl<T: Clone + 'static> Update<T> for MutableVec<T> {
    fn update<F>(&self, f: F) -> Box<dyn Fn() + 'static>
    where
        F: Fn(&Self) + 'static,
    {
        let state = self.clone();
        let cb = move || {
            (f(&state));
        };
        Box::new(cb)
    }

    fn update_with<F, E>(&self, f: F) -> Box<dyn Fn(E)>
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
