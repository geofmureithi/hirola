use std::{cell::RefCell, future::Future, marker::PhantomData, pin::Pin, rc::Rc};

use crate::{
    builder::{component::Component, ViewBuilder},
    generic_node::GenericNode,
    render::Error,
    templating::live_fragment::LiveFragment,
    view::View,
};
pub type SuspenseResult<T, E> = Result<Option<T>, E>;

pub struct Suspense<Res, G: GenericNode> {
    pub template: Box<dyn Fn(Res) -> ViewBuilder<G>>,
    pub future: Pin<Box<dyn Future<Output = Res>>>,
}

impl<Res: Default + 'static, G: GenericNode> Component<G> for Suspense<Res, G> {
    fn render(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        let template = self.template;
        struct State<G: GenericNode> {
            holder: LiveFragment<G>,
            current: Option<View<G>>,
        }

        impl<G: GenericNode> State<G> {
            fn new(parent: G) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: LiveFragment::new(parent),
                    current: None,
                }))
            }

            fn clear(&mut self) {
                let node = &mut self.holder;
                if let Some(frag) = &self.current {
                    for child in &frag.children().take() {
                        node.remove_child(&child.node());
                    }
                };
                self.current = None;
            }

            fn apply(&mut self, dom: ViewBuilder<G>) -> Result<(), Error> {
                let node = &mut self.holder;
                if let Some(current) = &self.current {
                    let children = current.children().take();
                    for child in children {
                        child.node().remove_self()
                    }
                }
                let view = dom.mount(&G::fragment())?;
                node.append_child(view.node().clone());
                self.current = Some(view);
                Ok(())
            }
        }
        impl<G: GenericNode> Drop for State<G> {
            fn drop(&mut self) {
                // self.clear();
            }
        }

        let state = State::new(view.node().clone());
        let binding = state.clone();
        let mut binding = binding.borrow_mut();
        // Apply loading
        binding.apply(template(Res::default()))?;
        let future = self.future;
        let fut = async move {
            let mut state = state.borrow_mut();

            let new_dom = template(future.await);
            state.apply(new_dom).unwrap();
        };
        // view.effect(fut);
        wasm_bindgen_futures::spawn_local(fut);
        Ok(())
    }
}
