//! Trait for describing how something should be rendered into DOM nodes.
use crate::{
    dom::Dom,
    generic_node::{DomType, GenericNode},
    templating::flow::{Indexed, IndexedProps},
};
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::{Filter, MutableSignalVec, MutableVec, SignalVec, SignalVecExt},
};
use std::{fmt::{Display, Debug}, iter::Enumerate, pin::Pin};

#[derive(Debug)]
pub enum Error {
    DomError(Box<dyn Debug>),
}

/// Trait for describing how something should be rendered into nodes.
pub trait Render {
    /// Called during the initial render when creating the nodes inside a dom.
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error>;
}

/// Does nothing
impl Render for () {
    fn render_into(self: Box<Self>, _dom: &Dom) -> Result<(), Error> {
        Ok(())
    }
}

impl Render for &str {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        let child = Dom::new_from_node(&DomType::text_node(*self));
        parent.append_child(child)?;
        Ok(())
    }
}

impl Render for String {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        let child = Dom::new_from_node(&DomType::text_node(&self));
        parent.append_child(child)?;
        Ok(())
    }
}

/// Renders `A`, then `B`
impl<A: Render, B: Render> Render for (A, B) {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        Box::new(self.0).render_into(parent)?;
        Box::new(self.1).render_into(parent)
    }
}

/// Renders `A`, then `B`, then `C`
impl<A: Render, B: Render, C: Render> Render for (A, B, C) {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        Box::new(self.0).render_into(parent)?;
        Box::new(self.1).render_into(parent)?;
        Box::new(self.2).render_into(parent)
    }
}

/// Renders `T` or nothing
impl<T: Render> Render for Option<T> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        match *self {
            None => Ok(()),
            Some(x) => Box::new(x).render_into(parent),
        }
    }
}

impl<T: Render> Render for Vec<T> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        for elem in *self {
            Box::new(elem).render_into(parent)?;
        }
        Ok(())
    }
}

impl<T: Display + Clone + 'static> Render for Mutable<T> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        let node = DomType::text_node(&self.get_cloned().to_string());
        let child = Dom::new_from_node(&node);
        let fut = self.signal_ref(move |e| node.update_inner_text(&e.to_string()));
        parent.effect(fut.to_future());
        parent.append_child(child).unwrap();
        Ok(())
    }
}

pub struct Mapped<T> {
    pub iter: Pin<Box<dyn SignalVec<Item = T>>>,
    callback: Box<dyn Fn(T) -> Dom>,
}

pub trait RenderMap {
    type Item;
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item>;
}

impl<T: Clone + 'static> RenderMap for MutableSignalVec<T> {
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, I: SignalVec<Item = T> + 'static, F: FnMut(&T) -> bool + 'static> RenderMap
    for Filter<I, F>
{
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, I: Iterator<Item = T>> RenderMap for Enumerate<I> {
    type Item = (usize, T);
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item> {
        let items = self.collect();
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(items).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static + PartialEq, I: SignalVec<Item = T> + Unpin + 'static> RenderMap
    for futures_signals::signal_vec::Enumerate<I>
{
    type Item = (ReadOnlyMutable<Option<usize>>, T);
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item> {
        Mapped {
            iter: Box::pin(self.to_signal_cloned().to_signal_vec()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static> RenderMap for Vec<T> {
    type Item = T;
    fn render_map(self, callback: impl Fn(Self::Item) -> Dom + 'static) -> Mapped<Self::Item> {
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(self).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: 'static + Clone> Render for Mapped<T> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
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
impl<O: Render, E: Render> Render for std::result::Result<O, E> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        match *self {
            Ok(o) => Box::new(o).render_into(parent),
            Err(e) => Box::new(e).render_into(parent),
        }
    }
}
