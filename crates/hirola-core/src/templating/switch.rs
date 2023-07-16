use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::{Signal, SignalExt};

use crate::{
    generic_node::{DomType, GenericNode},
    render::{Error, Render},
    dom::Dom,
};

pub struct Switch<S: Signal<Item = bool>, F>
where
    F: Fn(bool) -> Dom,
{
    pub signal: S,
    pub renderer: F,
}

impl<S, F> Render for Switch<S, F>
where
    F: Fn(bool) -> Dom + 'static,
    S: Signal<Item = bool> + 'static,
{
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        let marker = DomType::marker();
        parent.node().append_child(&marker);
        let state = State::new(parent.node().clone(), marker);
        let renderer = self.renderer;
        struct State<DomType> {
            holder: DomType,
            marker: DomType,
            current: Option<Dom>,
        }

        impl State<DomType> {
            fn new(element: DomType, marker: DomType) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: element,
                    current: None,
                    marker,
                }))
            }

            fn clear(&mut self) {
                let node = &mut self.holder;
                if let Some(frag) = &self.current {
                    for child in &frag.children().take() {
                        log::debug!("Result for remove {:?}", node.remove_child(&child.node()));
                    }
                };
                self.current = None;
            }

            fn apply(&mut self, dom: Dom) {
                self.clear();
                let node = &self.holder;
                let dom = dom.mount(&DomType::fragment()).unwrap();
                node.insert_child_before(&dom.node(), Some(&self.marker));
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
