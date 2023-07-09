//! Iteration utility components for [`dom`](crate::html).
//!
//! Iteration can be either _"keyed"_ or _"non keyed"_.
//! Use the [`Keyed`] and [`Indexed`] utility components respectively.

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::future::ready;
use std::hash::Hash;
use std::marker::PhantomData;
use std::mem;
use std::rc::Rc;

use discard::{Discard, DiscardOnDrop};
use futures_signals::signal_vec::{MutableSignalVec, MutableVec, SignalVec, SignalVecExt, VecDiff};
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::window;

use crate::builder::component::Component;
use crate::builder::ViewBuilder;
use crate::generic_node::GenericNode;
use crate::prelude::*;
use crate::render::{Error, Render};
use crate::view::View;
// Props for [`Indexed`].
///
//#[derive(Debug)]
pub struct IndexedProps<T, G: GenericNode, I: SignalVec<Item = T> + Unpin, F>
where
    F: Fn(T) -> ViewBuilder<G>,
{
    pub iterable: I,
    pub template: F,
}

/// Non keyed iteration (or keyed by index). Use this instead of directly rendering an array of [`TemplateResult`]s.
/// Using this will minimize re-renders instead of re-rendering every single node on every state change.
///
/// For keyed iteration, see [`Keyed`].
///
/// # Example
/// ```ignore
/// use hirola::prelude::*;
///
/// let count = MutableVec::new_with_values(vec![1, 2]);
///
/// let res = html! {
///      <Indexed
///         props={IndexedProps {
///             iterable: count,
///             template: move |item| html! {
///                <li>{ item }</li>
///             },
///         }}
///     />
/// };
/// # let _ : Dom = res;
/// ```
// #[component]
pub struct Indexed<T, I: SignalVec<Item = T> + Unpin, F, G: GenericNode>
where
    F: Fn(T) -> ViewBuilder<G>,
{
    pub props: IndexedProps<T, G, I, F>,
}

impl<T, F, I, G: GenericNode> Component<G> for Indexed<T, I, F, G>
where
    T: 'static,
    I: 'static + SignalVec<Item = T> + Unpin,
    F: Fn(T) -> ViewBuilder<G> + 'static,
{
    fn render(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let props = self.props;
        let template = props.template;
        let iterable = SignalVecExt::map(props.iterable, move |item| {
            template(item).mount(&G::fragment()).unwrap()
        });

        let marker = G::marker();

        struct State<G: GenericNode> {
            element: G,
            marker: G,
            is_inserted: bool,
            children: Vec<View<G>>,
        }

        impl<G: GenericNode> State<G> {
            fn new(element: G, marker: G) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    element,
                    marker,
                    is_inserted: false,
                    children: vec![],
                }))
            }

            fn clear(&mut self) {
                for dom in self.children.drain(..) {
                    self.element.remove_child(&dom.node());
                    drop(dom)
                }
            }

            fn insert_at(&self, new_index: usize, child: &G) {
                if let Some(dom) = self.children.get(new_index) {
                    self.element.insert_child_before(child, Some(&dom.node()));
                } else {
                    self.element.insert_child_before(child, Some(&self.marker));
                }
            }

            // TODO verify that this will drop `children`
            fn process_change(&mut self, change: VecDiff<View<G>>) {
                match change {
                    VecDiff::Replace { values } => {
                        self.clear();

                        self.children = values;

                        let is_inserted = self.is_inserted;

                        // TODO use createDocumentFragment ?
                        for dom in self.children.iter_mut() {
                            self.element
                                .insert_child_before(&dom.node(), Some(&self.marker));
                        }
                    }

                    VecDiff::InsertAt { index, value } => {
                        self.insert_at(index, &value.node());
                        self.children.insert(index, value);
                    }

                    VecDiff::Push { value } => {
                        let marker = self.marker.clone();
                        self.element
                            .insert_child_before(value.node(), Some(&marker));
                        self.children.push(value);
                    }

                    VecDiff::UpdateAt { index, mut value } => {
                        let dom = &mut self.children[index];
                        self.element.replace_child(value.node(), &self.marker);
                        ::std::mem::swap(dom, &mut value);
                    }

                    VecDiff::Move {
                        old_index,
                        new_index,
                    } => {
                        let value = self.children.remove(old_index);

                        self.insert_at(new_index, value.node());

                        self.children.insert(new_index, value);
                    }

                    VecDiff::RemoveAt { index } => {
                        let dom = self.children.remove(index);
                        // self.element.remove_child(&dom.node());
                        let children = dom.children().take();
                        for child in children {
                            child.node().remove_self()
                        }
                        drop(dom)
                    }

                    VecDiff::Pop {} => {
                        let dom = self.children.pop().unwrap_throw();
                        // self.element.remove_child(&dom.node());
                        let children = dom.children().take();
                        for child in children {
                            child.node().remove_self()
                        }
                        drop(dom)
                    }

                    VecDiff::Clear {} => {
                        self.clear();
                    }
                }
            }
        }

        view.append_child(View::new_from_node(&marker.clone()))
            .unwrap();

        let state = State::new(view.node().clone(), marker);

        impl<G: GenericNode> Drop for State<G> {
            fn drop(&mut self) {
                self.clear();
            }
        }

        let fut = iterable.for_each(move |change| {
            let mut state = state.borrow_mut();
            state.process_change(change);
            ready(())
        });
        wasm_bindgen_futures::spawn_local(fut);
        // view.effect(fut);
        Ok(())
    }
}
