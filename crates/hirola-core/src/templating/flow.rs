//! Iteration utility components for [`dom`](crate::generic_node::dom_node).
//!
//! Iteration can be either _"keyed"_ or _"non keyed"_.
//! Use the [`Keyed`] and [`Indexed`] utility components respectively.
use crate::builder::component::Component;
use crate::builder::DomBuilder;
use crate::generic_node::{DomType, GenericNode};
use crate::render::Error;
use crate::view::View;
use futures_signals::signal_vec::{SignalVec, SignalVecExt, VecDiff};
use std::cell::RefCell;
use std::future::ready;
use std::rc::Rc;
use wasm_bindgen::UnwrapThrowExt;

/// Props for [`Indexed`].
///
#[derive(Debug)]
pub struct IndexedProps<T, I: SignalVec<Item = T> + Unpin, F>
where
    F: Fn(T) -> DomBuilder,
{
    pub iterable: I,
    pub template: F,
}

/// Non keyed iteration (or keyed by index). Use this instead of directly rendering an array of [`DomBuilder`]s.
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
pub struct Indexed<T, I: SignalVec<Item = T> + Unpin, F>
where
    F: Fn(T) -> DomBuilder,
{
    pub props: IndexedProps<T, I, F>,
}

impl<T, F, I> Component for Indexed<T, I, F>
where
    T: 'static + Clone,
    I: 'static + SignalVecExt<Item = T> + Unpin,
    F: Fn(T) -> DomBuilder + 'static,
{
    fn render(self: Box<Self>, view: &View) -> Result<(), Error> {
        let props = self.props;
        let template = props.template;

        let iterable = SignalVecExt::map(props.iterable, move |item| {
            template(item).mount(&DomType::fragment()).unwrap()
        });

        let marker = DomType::marker();

        struct State {
            element: DomType,
            marker: DomType,
            children: Vec<View>,
        }

        impl State {
            fn new(element: DomType, marker: DomType) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    element,
                    marker,
                    children: vec![],
                }))
            }

            fn clear(&mut self) {
                for dom in self.children.drain(..) {
                    self.element.remove_child(&dom.node());
                    drop(dom)
                }
            }

            fn insert_at(&self, new_index: usize, child: &DomType) {
                if let Some(dom) = self.children.get(new_index) {
                    self.element.insert_child_before(child, Some(&dom.node()));
                } else {
                    self.element.insert_child_before(child, Some(&self.marker));
                }
            }

            // TODO verify that this will drop `children`
            fn process_change(&mut self, change: VecDiff<View>) {
                match change {
                    VecDiff::Replace { values } => {
                        self.clear();
                        self.children = values;
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
                        let children = dom.children().take();
                        for child in children {
                            child.node().remove_self()
                        }
                        drop(dom)
                    }

                    VecDiff::Pop {} => {
                        let dom = self.children.pop().unwrap_throw();
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

        let fut = iterable.for_each(move |change| {
            let mut state = state.borrow_mut();
            state.process_change(change);
            ready(())
        });
        view.effect(fut);
        Ok(())
    }
}
