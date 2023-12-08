use crate::{
    generic_node::GenericNode,
    render::{Error, Render},
    BoxedLocal,
};
use futures_util::future::FutureExt;
use std::{cell::RefCell, future::Future, pin::Pin, rc::Rc};

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

pub struct Suspense<Res, G> {
    pub template: Box<dyn Fn(Res) -> G>,
    pub future: Pin<Box<dyn Future<Output = Res>>>,
}

impl<Res: Default + 'static, N: GenericNode> Render<N> for Suspense<Res, N> {
    fn render_into(self: Box<Self>, parent: &N) -> Result<(), Error> {
        let template = self.template;
        struct State<N> {
            holder: N,
            current: Option<N>,
        }

        impl<N: GenericNode> State<N> {
            fn new(parent: N) -> Rc<RefCell<Self>> {
                Rc::new(RefCell::new(State {
                    holder: parent,
                    current: None,
                }))
            }

            fn clear(&mut self) {
                {
                    let node = &mut self.holder;
                    if let Some(frag) = &self.current {
                        for child in &frag.children().take() {
                            node.remove_child(child);
                        }
                    };
                }
                self.current = None;
            }

            fn apply(&mut self, dom: N) -> Result<(), Error> {
                self.clear();
                let node = &mut self.holder;
                let frag = N::fragment();
                frag.append_child(&dom);
                node.append_child(&frag);
                self.current = Some(frag);
                Ok(())
            }
        }

        let state = State::new(parent.clone());
        let binding = state.clone();
        let mut binding = binding.borrow_mut();
        // Apply loading
        binding.apply(template(Res::default()))?;
        let future = self.future;
        let fut = async move {
            let new_dom = template(future.await);
            let mut state = state.borrow_mut();
            state.apply(new_dom).unwrap();
        };
        parent.effect(fut);
        Ok(())
    }
}
