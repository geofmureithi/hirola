use std::{cell::RefCell, future::Future};

use discard::Discard;
use futures_signals::CancelableFutureHandle;
use wasm_bindgen::{prelude::Closure, JsValue};
use crate::{
    generic_node::{EventListener, GenericNode}, BoxedLocal,
};


pub enum DomSideEffect {
    UnMounted(BoxedLocal<()>),
    Mounted(CancelableFutureHandle),
}
pub struct View<G: GenericNode> {
    node: G,
    side_effects: RefCell<Vec<DomSideEffect>>,
    event_handlers: RefCell<Vec<Closure<EventListener>>>,
    children: RefCell<Vec<View<G>>>,
}

impl<G: GenericNode> View<G> {
    pub fn append_child(&self, child: View<G>) -> Result<(), JsValue> {
        let new_node = self.node.append_child(&child.node);
        self.children.borrow_mut().push(child);
        Ok(())
    }

    pub fn children(&self) -> &RefCell<Vec<View<G>>> {
        &self.children
    }

    pub fn node(&self) -> &G {
        &self.node
    }

    pub fn new_from_node(node: &G) -> View<G> {
        Self {
            node: node.clone(),
            children: RefCell::new(Vec::new()),
            event_handlers: RefCell::new(Vec::new()),
            side_effects: RefCell::new(Vec::new()),
        }
    }

    #[inline]
    pub fn event(&self, name: &str, handler: Box<EventListener>) {
        let closure = self.node.event(name, handler);
        if let Some(closure) = closure {
            self.event_handlers.borrow_mut().push(closure);
        }
    }
    #[inline]
    pub fn effect(&self, fut: impl Future<Output = ()> + 'static) {
        self.side_effects
            .borrow_mut()
            .push(DomSideEffect::UnMounted(Box::pin(fut)))
    }

    #[inline]
    pub fn discard(&mut self) {
        let _cleanup: Vec<()> = std::mem::take(&mut self.event_handlers)
            .into_inner()
            .into_iter()
            .map(|c| c.forget())
            .collect();
        let effects = std::mem::take(&mut self.side_effects);
        let _cleanup: Vec<()> = effects
            .into_inner()
            .into_iter()
            .map(|e| match e {
                DomSideEffect::Mounted(e) => e.discard(),
                DomSideEffect::UnMounted(_) => {
                    log::warn!("Dropping a side effect that was not mounted")
                }
            })
            .collect();
    }
}

impl<G: GenericNode> Drop for View<G> {
    fn drop(&mut self) {
        self.discard()
    }
}
