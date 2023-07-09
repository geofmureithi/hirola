//! Trait for describing how something should be rendered into DOM nodes.

use std::{
    fmt::{self, Display},
    iter::Enumerate,
    ops::Range,
    pin::Pin,
};

use discard::DiscardOnDrop;
use futures::stream::LocalBoxStream;
use futures_signals::{
    signal::{Mutable, ReadOnlyMutable, SignalExt},
    signal_vec::{
        Filter, FilterMap, Map, MutableSignalVec, MutableVec, SignalVec, SignalVecExt, VecDiff,
    },
};
use wasm_bindgen::JsValue;

use crate::{
    builder::ViewBuilder,
    generic_node::{DomNode, GenericNode},
    templating::flow::{Indexed, IndexedProps},
    view::View,
};

#[derive(Debug)]
pub enum Error {
    DomError(JsValue),
}

/// Trait for describing how something should be rendered into nodes.
pub trait Render<G: GenericNode> {
    /// Called during the initial render when creating the nodes inside a view.
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error>;

    // /// Called when the node should be updated with new state.
    // /// The default implementation of this will replace the child node completely with the result of calling `render` again.
    // /// Another implementation might be better suited to some specific types.
    // /// For example, text nodes can simply replace the inner text instead of recreating a new node.
    // ///
    // /// Returns the new node. If the node is reused instead of replaced, the returned node is simply the node passed in.
    // fn update_node(&self, parent: &G, node: &G) {

    // }
}

/// Does nothing
impl<G: GenericNode> Render<G> for () {
    fn render_into(self: Box<Self>, _view: &View<G>) -> Result<(), Error> {
        Ok(())
    }
}

impl<G: GenericNode> Render<G> for &str {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let child = View::new_from_node(&G::text_node(*self));
        view.append_child(child);
        Ok(())
    }
}

impl<G: GenericNode> Render<G> for String {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let child = View::new_from_node(&G::text_node(&self));
        view.append_child(child);
        Ok(())
    }
}

/// Renders `A`, then `B`
impl<A: Render<G>, B: Render<G>, G: GenericNode> Render<G> for (A, B) {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        Box::new(self.0).render_into(view)?;
        Box::new(self.1).render_into(view)
    }
}

/// Renders `A`, then `B`, then `C`
impl<A: Render<G>, B: Render<G>, C: Render<G>, G: GenericNode> Render<G> for (A, B, C) {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        Box::new(self.0).render_into(view)?;
        Box::new(self.1).render_into(view)?;
        Box::new(self.2).render_into(view)
    }
}

/// Renders `T` or nothing
impl<T: Render<G>, G: GenericNode> Render<G> for Option<T> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        match *self {
            None => Ok(()),
            Some(x) => Box::new(x).render_into(view),
        }
    }
}

impl<T: Render<G>, G: GenericNode> Render<G> for Vec<T> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        for elem in *self {
            Box::new(elem).render_into(view)?;
        }
        Ok(())
    }
}

impl<T: Display + Clone + 'static, G: GenericNode> Render<G> for Mutable<T> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let node = G::text_node(&self.get_cloned().to_string());
        let v = View::new_from_node(&node);

        let fut = self.signal_ref(move |e| node.update_inner_text(&e.to_string()));
        wasm_bindgen_futures::spawn_local(fut.to_future());
        // view.effect(fut.to_future());
        view.append_child(v).unwrap();
        Ok(())
    }
}

pub struct Mapped<T, G> {
    pub iter: Pin<Box<dyn SignalVec<Item = T>>>,
    callback: Box<dyn Fn(T) -> ViewBuilder<G>>,
}

pub trait RenderMap<G> {
    type Item;
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G>;
}

impl<T: Clone + 'static, G: GenericNode> RenderMap<G> for MutableSignalVec<T> {
    type Item = T;
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<
        T: Clone + 'static,
        G: GenericNode,
        I: SignalVec<Item = T> + 'static,
        F: FnMut(&T) -> bool + 'static,
    > RenderMap<G> for Filter<I, F>
{
    type Item = T;
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, G: GenericNode, I: Iterator<Item = T>> RenderMap<G> for Enumerate<I> {
    type Item = (usize, T);
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G> {
        let items = self.collect();
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(items).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static + PartialEq, G: GenericNode, I: SignalVec<Item = T> + Unpin + 'static>
    RenderMap<G> for futures_signals::signal_vec::Enumerate<I>
{
    type Item = (ReadOnlyMutable<Option<usize>>, T);
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(self.to_signal_cloned().to_signal_vec()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, G: GenericNode> RenderMap<G> for Vec<T> {
    type Item = T;
    fn render_map(
        self,
        callback: impl Fn(Self::Item) -> ViewBuilder<G> + 'static,
    ) -> Mapped<Self::Item, G> {
        Mapped {
            iter: Box::pin(MutableVec::new_with_values(self).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: 'static, G: GenericNode> Render<G> for Mapped<T, G> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let template = {
            let props = IndexedProps {
                iterable: self.iter,
                template: self.callback,
            };
            let indexed = Indexed { props };
            ViewBuilder::Component(Box::new(indexed))
        };
        Box::new(template).render_into(view)?;
        Ok(())
    }
}

/// Renders `O` or `E`
impl<O: Render<G>, E: Render<G>, G: GenericNode> Render<G> for std::result::Result<O, E> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        match *self {
            Ok(o) => Box::new(o).render_into(view),
            Err(e) => Box::new(e).render_into(view),
        }
    }
}
