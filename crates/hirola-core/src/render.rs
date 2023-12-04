//! Trait for describing how components and other custom types should be rendered into DOM nodes.
use crate::{
    generic_node::GenericNode,
    templating::flow::{Indexed, IndexedProps},
};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::{Filter, MutableSignalVec, MutableVec, SignalVec, SignalVecExt},
};
use std::{
    fmt::{Debug, Display},
    iter::Enumerate,
    pin::Pin,
};

#[derive(Debug)]
pub enum Error {
    DomError(Box<dyn Debug>),
}

/// Trait for describing how something should be rendered into nodes.
pub trait Render<N: GenericNode> {
    /// Called during the initial render when creating the nodes inside a dom.
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error>;
}

/// Does nothing
impl<N: GenericNode> Render<N> for () {
    fn render_into(self: Box<Self>, _dom: &N) -> Result<(), Error> {
        Ok(())
    }
}

impl<N: GenericNode> Render<N> for &str {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let child = &N::text_node(*self);
        parent.append_child(child);
        Ok(())
    }
}

impl<N: GenericNode> Render<N> for String {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let child = &N::text_node(&self);
        parent.append_child(child);
        Ok(())
    }
}

impl<N: GenericNode> Render<N> for &String {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let child = &N::text_node(&self);
        parent.append_child(child);
        Ok(())
    }
}

/// Renders `A`, then `B`
impl<A: Render<N>, B: Render<N>, N: GenericNode> Render<N> for (A, B) {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        Box::new(self.0).render_into(parent)?;
        Box::new(self.1).render_into(parent)
    }
}

/// Renders `A`, then `B`, then `C`
impl<A: Render<N>, B: Render<N>, C: Render<N>, N: GenericNode> Render<N> for (A, B, C) {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        Box::new(self.0).render_into(parent)?;
        Box::new(self.1).render_into(parent)?;
        Box::new(self.2).render_into(parent)
    }
}

/// Renders `T` or nothing
impl<T: Render<N>, N: GenericNode> Render<N> for Option<T> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        match *self {
            None => Ok(()),
            Some(x) => Box::new(x).render_into(parent),
        }
    }
}

impl<T: Render<N>, N: GenericNode> Render<N> for Vec<T> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        for elem in *self {
            Box::new(elem).render_into(parent)?;
        }
        Ok(())
    }
}

impl<T: Render<N>, N: GenericNode> Render<N> for Box<T> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        (*self).render_into(parent)?;
        Ok(())
    }
}

impl<T: Display + Clone + 'static, N: GenericNode> Render<N> for Mutable<T> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let child = N::text_node(&self.get_cloned().to_string());
        parent.append_child(&child);
        let fut = self.signal_ref(move |e| child.update_inner_text(&e.to_string()));
        parent.effect(fut.to_future());
        Ok(())
    }
}

pub struct Mapped<T, G: GenericNode> {
    pub iter: Pin<Box<dyn SignalVec<Item = T>>>,
    callback: Box<dyn Fn(T) -> G>,
}

pub trait RenderMap<G: GenericNode> {
    type Item;
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G>;
}

impl<T: Clone + 'static, G: GenericNode> RenderMap<G> for MutableSignalVec<T> {
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<
        T: Clone + 'static,
        I: SignalVec<Item = T> + 'static,
        F: FnMut(&T) -> bool + 'static,
        G: GenericNode,
    > RenderMap<G> for Filter<I, F>
{
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, I: Iterator<Item = T>, G: GenericNode> RenderMap<G> for Enumerate<I> {
    type Item = (usize, T);
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
        let items = self.collect();
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(items).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static + PartialEq, I: SignalVec<Item = T> + Unpin + 'static, G: GenericNode>
    RenderMap<G> for futures_signals::signal_vec::Enumerate<I>
{
    type Item = (ReadOnlyMutable<Option<usize>>, T);
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self.to_signal_cloned().to_signal_vec()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, G: GenericNode> RenderMap<G> for Vec<T> {
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(self).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: 'static + Clone, N: GenericNode> Render<N> for Mapped<T, N> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let template = {
            let props = IndexedProps {
                iterable: self.iter,
                template: self.callback,
            };
            let indexed = Indexed { props };
            indexed
        };
        Box::new(template).render_into(parent)?;
        Ok(())
    }
}

/// Renders `O` or `E`
impl<O: Render<N>, E: Render<N>, N: GenericNode> Render<N> for std::result::Result<O, E> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        match *self {
            Ok(o) => Box::new(o).render_into(parent),
            Err(e) => Box::new(e).render_into(parent),
        }
    }
}
