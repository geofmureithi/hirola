use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::{Signal, SignalExt};

use crate::{
    builder::{component::Component, DomBuilder},
    generic_node::{DomType, GenericNode},
    render::Error,
    view::View,
};

pub struct Switch<S: Signal<Item = bool>, F>
where
    F: Fn(bool) -> DomBuilder,
{
    pub signal: S,
    pub renderer: F,
}

impl<S, F> Component for Switch<S, F>
where
    F: Fn(bool) -> DomBuilder + 'static,
    S: Signal<Item = bool> + 'static,
{
    fn render(self: Box<Self>, view: &View) -> Result<(), Error> {
        let marker = DomType::marker();
        view.node().append_child(&marker);
        let state = State::new(view.node().clone(), marker);
        let renderer = self.renderer;
        struct State<DomType> {
            holder: DomType,
            marker: DomType,
            current: Option<View>,
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

            fn apply(&mut self, dom: DomBuilder) {
                self.clear();
                let node = &self.holder;
                let view = dom.mount(&DomType::fragment()).unwrap();
                node.insert_child_before(&view.node(), Some(&self.marker));
                self.current = Some(view);
            }
        }
        let fut = self.signal.for_each(move |val| {
            let mut state = state.borrow_mut();
            state.apply(renderer(val));

            async {}
        });
        view.effect(fut);
        Ok(())
    }
}
