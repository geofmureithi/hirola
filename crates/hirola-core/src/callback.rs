use crate::prelude::DomNode;

pub trait StateReduce<T> {
    fn mut_callback<F, E>(&self, f: F) -> Box<dyn Fn(E) -> ()>
    where
        F: Fn(&T, E) -> T + 'static;
}

pub trait Mixin {
    fn mixin(&self, namespace: &str, node: DomNode);
}

impl<T> Mixin for T
where
    T: Fn(DomNode) -> (),
{
    fn mixin(&self, _ns: &str, node: DomNode) {
        (&self)(node)
    }
}

pub trait State: Clone {
    // Get a callback that allows interacting with state
    fn callback<F, E>(&self, f: F) -> Box<dyn Fn(E) -> ()>
    where
        F: Fn(&Self, E) -> () + 'static,
        Self: 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            f(&state, e);
        };
        Box::new(cb)
    }
}
