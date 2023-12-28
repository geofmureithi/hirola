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

pub struct MappedVec<T, G: GenericNode> {
    pub iter: Pin<Box<dyn SignalVec<Item = T>>>,
    callback: Box<dyn Fn(T) -> G>,
}

// pub struct Mapped<T, G: GenericNode> {
//     pub signal: Pin<Box<dyn Signal<Item = T>>>,
//     callback: Box<dyn Fn(T) -> G>,
// }

pub trait MapRender<G: GenericNode> {
    type Item;
    type Output;
    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> Self::Output;
}

impl<T: Clone + 'static, G: GenericNode> MapRender<G> for MutableSignalVec<T> {
    type Item = T;
    type Output = MappedVec<Self::Item, G>;
    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> MappedVec<Self::Item, G> {
        MappedVec {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

// impl<T: Clone + 'static, G: GenericNode> MapRender<G> for Mutable<T> {
//     type Item = T;
//     type Output = Mapped<Self::Item, G>;

//     fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> Mapped<Self::Item, G> {
//         Mapped {
//             signal: Box::pin(self.signal_cloned()),
//             callback: Box::new(callback),
//         }
//     }
// }

impl<
        T: Clone + 'static,
        I: SignalVec<Item = T> + 'static,
        F: FnMut(&T) -> bool + 'static,
        G: GenericNode,
    > MapRender<G> for Filter<I, F>
{
    type Item = T;
    type Output = MappedVec<Self::Item, G>;

    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> MappedVec<Self::Item, G> {
        MappedVec {
            iter: Box::pin(self),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, I: Iterator<Item = T>, G: GenericNode> MapRender<G> for Enumerate<I> {
    type Item = (usize, T);
    type Output = MappedVec<Self::Item, G>;

    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> MappedVec<Self::Item, G> {
        let items = self.collect();
        MappedVec {
            iter: Box::pin(MutableVec::new_with_values(items).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static + PartialEq, I: SignalVec<Item = T> + Unpin + 'static, G: GenericNode>
    MapRender<G> for futures_signals::signal_vec::Enumerate<I>
{
    type Item = (ReadOnlyMutable<Option<usize>>, T);
    type Output = MappedVec<Self::Item, G>;

    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> MappedVec<Self::Item, G> {
        MappedVec {
            iter: Box::pin(self.to_signal_cloned().to_signal_vec()),
            callback: Box::new(callback),
        }
    }
}

impl<T: Clone + 'static, G: GenericNode> MapRender<G> for Vec<T> {
    type Item = T;
    type Output = MappedVec<Self::Item, G>;

    fn map_render(self, callback: impl Fn(Self::Item) -> G + 'static) -> MappedVec<Self::Item, G> {
        MappedVec {
            iter: Box::pin(MutableVec::new_with_values(self).signal_vec_cloned()),
            callback: Box::new(callback),
        }
    }
}

impl<T: 'static + Clone, N: GenericNode> Render<N> for MappedVec<T, N> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let template = {
            #[allow(clippy::type_complexity)]
            let props: IndexedProps<
                T,
                Pin<Box<dyn SignalVec<Item = T>>>,
                Box<dyn Fn(T) -> N>,
                N,
            > = IndexedProps {
                iterable: self.iter,
                template: self.callback,
            };

            Indexed { props }
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
