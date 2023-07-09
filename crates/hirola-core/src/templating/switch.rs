use std::{cell::RefCell, rc::Rc};

use futures_signals::signal::{Signal, SignalExt};

use crate::{
    builder::{component::Component, ViewBuilder},
    generic_node::GenericNode,
    render::Error,
    view::View,
};

pub struct Switch<G, S: Signal<Item = bool>, F>
where
    F: Fn(bool) -> ViewBuilder<G>,
{
    pub signal: S,
    pub renderer: F,
}

impl<G: GenericNode, S, F> Component<G> for Switch<G, S, F>
where
    F: Fn(bool) -> ViewBuilder<G> + 'static,
    S: Signal<Item = bool> + 'static,
{
    fn render(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let marker = G::marker();
        view.node().append_child(&marker);
        let state = State::new(view.node().clone(), marker);
        let renderer = self.renderer;
        struct State<G: GenericNode> {
            holder: G,
            marker: G,
            current: Option<View<G>>,
        }

        impl<G: GenericNode> State<G> {
            fn new(element: G, marker: G) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: element,
                    current: None,
                    marker
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

            fn apply(&mut self, dom: ViewBuilder<G>) {
                self.clear();
                let node = &self.holder;
                let view = dom.mount(&G::fragment()).unwrap();
                node.insert_child_before(&view.node(), Some(&self.marker));
                self.current = Some(view);
            }
        }
        impl<G: GenericNode> Drop for State<G> {
            fn drop(&mut self) {
                // self.clear();
            }
        }
        let fut = self.signal.for_each(move |val| {
            let mut state = state.borrow_mut();
            state.apply(renderer(val));

            async {}
        });
        wasm_bindgen_futures::spawn_local(fut);
        Ok(())
    }
}
