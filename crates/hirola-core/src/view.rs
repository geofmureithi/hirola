use crate::{
    generic_node::{DomType, EventListener, GenericNode},
    spawn, BoxedLocal,
};
use discard::{Discard, DiscardOnDrop};
use futures_signals::CancelableFutureHandle;
use std::{cell::RefCell, future::Future, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsValue};

pub enum DomSideEffect {
    UnMounted(BoxedLocal<()>),
    Mounted(CancelableFutureHandle),
}

#[derive(Clone)]
pub struct View {
    node: DomType,
    pub side_effects: Rc<RefCell<Vec<DomSideEffect>>>,
    event_handlers: Rc<RefCell<Vec<Closure<EventListener>>>>,
    children: RefCell<Vec<View>>,
}

impl View {
    pub fn append_child(&self, child: View) -> Result<(), JsValue> {
        self.node.append_child(&child.node);
        self.children.borrow_mut().push(child);
        Ok(())
    }

    pub fn children(&self) -> &RefCell<Vec<View>> {
        &self.children
    }

    pub fn node(&self) -> &DomType {
        &self.node
    }

    pub fn new_from_node(node: &DomType) -> View {
        Self {
            node: node.clone(),
            children: Default::default(),
            event_handlers: Default::default(),
            side_effects: Default::default(),
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
    pub fn attribute(&self, name: &str, value: &str) {
        self.node.set_attribute(name, value);
    }
    #[inline]
    pub fn effect(&self, future: impl Future<Output = ()> + 'static) {
        self.side_effects
            .borrow_mut()
            .push(DomSideEffect::Mounted(DiscardOnDrop::leak(spawn(future))));
    }

    #[inline]
    pub fn discard(&mut self) {
        let _cleanup: Vec<()> = self
            .event_handlers
            .take()
            .into_iter()
            .map(|c| c.forget())
            .collect();

        let _cleanup: Vec<()> = self
            .side_effects
            .take()
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

impl Drop for View {
    fn drop(&mut self) {
        self.discard()
    }
}
