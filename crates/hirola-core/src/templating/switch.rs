use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::{Signal, SignalExt};

use crate::{
    generic_node::GenericNode,
    render::{Error, Render},
};

pub struct Switch<S: Signal<Item = bool>, F, G>
where
    F: Fn(bool) -> G,
{
    pub signal: S,
    pub renderer: F,
}

impl<S, F, N: GenericNode> Render<N> for Switch<S, F, N>
where
    F: Fn(bool) -> N + 'static,
    S: Signal<Item = bool> + 'static,
{
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let marker = N::marker();
        parent.append_child(&marker);
        let state = State::new(parent.clone(), marker);
        let renderer = self.renderer;
        struct State<DomType: GenericNode> {
            holder: DomType,
            marker: DomType,
            current: Option<DomType>,
        }

        impl<DomType: GenericNode> State<DomType> {
            fn new(element: DomType, marker: DomType) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: element,
                    current: None,
                    marker,
                }))
            }

            fn clear(&mut self) {
                {
                    let node = &mut self.holder;
                    if let Some(frag) = &self.current {
                        for child in &frag.children().take() {
                            node.remove_child(&child)
                        }
                    };
                }
                self.current = None;
            }

            fn apply(&mut self, dom: DomType) {
                self.clear();
                let node = &self.holder;
                node.insert_child_before(&dom, Some(&self.marker));
                self.current = Some(dom);
            }
        }
        let fut = self.signal.for_each(move |val| {
            let mut state = state.borrow_mut();
            state.apply(renderer(val));

            async {}
        });
        parent.effect(fut);
        Ok(())
    }
}
