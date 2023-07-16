use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};
use futures_util::future::FutureExt;
use crate::{
    generic_node::{GenericNode, DomType},
    render::{Error, Render},
    dom::Dom, BoxedLocal,
};

#[derive(Debug, Default)]
pub enum SuspenseResult<Res> {
    #[default]
    Loading,
    Ready(Res),
}

pub trait Suspend {
    type Result;
    fn suspend(self) -> BoxedLocal<SuspenseResult<Self::Result>>;
}

impl<F, Res> Suspend for F
where
    F: FutureExt<Output = Res> + 'static,
{
    type Result = Res;
    fn suspend(self) -> BoxedLocal<SuspenseResult<Self::Result>> {
        Box::pin(self.map(|res| SuspenseResult::Ready(res)))
    }
}

pub struct Suspense<Res> {
    pub template: Box<dyn Fn(Res) -> Dom>,
    pub future: Pin<Box<dyn Future<Output = Res>>>,
}

impl<Res: Default + 'static> Render for Suspense<Res> {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        let template = self.template;
        struct State{
            holder: DomType,
            current: Option<Dom>,
        }

        impl State {
            fn new(parent: DomType) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: parent,
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

            fn apply(&mut self, dom: Dom) -> Result<(), Error> {
                self.clear();
                let node = &mut self.holder;
                let dom = dom.mount(&DomType::fragment())?;
                node.append_child(&dom.node());
                self.current = Some(dom);
                Ok(())
            }
        }

        let state = State::new(parent.node().clone());
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
        parent.effect(fut);
        Ok(())
    }
}
