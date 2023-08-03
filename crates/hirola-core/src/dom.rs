use crate::{
    generic_node::{DomType, GenericNode},
    render::{Error, Render},
    spawn, BoxedLocal,
};
use discard::{Discard, DiscardOnDrop};
use futures_signals::CancelableFutureHandle;
use std::{cell::RefCell, future::Future, rc::Rc};
#[cfg(feature = "dom")]
use wasm_bindgen::JsCast;
#[cfg(feature = "dom")]
use web_sys::HtmlElement;

#[cfg(feature = "dom")]
use wasm_bindgen::prelude::Closure;

#[cfg(feature = "dom")]
use crate::generic_node::EventListener;

pub enum DomSideEffect {
    UnMounted(BoxedLocal<()>),
    Mounted(CancelableFutureHandle),
}

#[derive(Clone)]
pub struct Dom {
    node: DomType,
    pub side_effects: Rc<RefCell<Vec<DomSideEffect>>>,
    #[cfg(feature = "dom")]
    event_handlers: Rc<RefCell<Vec<Closure<EventListener>>>>,
    children: RefCell<Vec<Dom>>,
}

impl Dom {
    pub fn new() -> Dom {
        Dom::new_from_node(&DomType::fragment())
    }

    pub fn element(tag: &str) -> Dom {
        Dom::new_from_node(&DomType::element(tag))
    }

    pub fn text(tag: &str) -> Dom {
        Dom::new_from_node(&DomType::text_node(tag))
    }

    pub fn append_child(&self, child: Dom) -> Result<(), Error> {
        self.node.append_child(&child.node);
        self.children.borrow_mut().push(child);
        Ok(())
    }

    pub fn children(&self) -> &RefCell<Vec<Dom>> {
        &self.children
    }

    pub fn node(&self) -> &DomType {
        &self.node
    }

    pub fn new_from_node(node: &DomType) -> Dom {
        Self {
            node: node.clone(),
            children: Default::default(),
            #[cfg(feature = "dom")]
            event_handlers: Default::default(),
            side_effects: Default::default(),
        }
    }

    #[cfg(feature = "dom")]
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

    pub fn append_render(&self, render: impl Render + 'static) {
        Box::new(render).render_into(&self).unwrap();
    }

    #[inline]
    pub fn discard(&mut self) {
        #[cfg(feature = "dom")]
        {
            let _cleanup: Vec<()> = self
                .event_handlers
                .take()
                .into_iter()
                .map(|c| c.forget())
                .collect();
        }
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

    pub fn mount(self, node: &DomType) -> Result<Dom, Error> {
        let dom = Dom::new_from_node(node);
        Box::new(self).render_into(&dom)?;
        Ok(dom)
    }

    pub fn inner_html(&self) -> String {
        #[cfg(feature = "dom")]
        {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element = document.create_element("div").unwrap();

            let dom = crate::render_to(self.clone(), &element.try_into().unwrap()).unwrap();
            return dom
                .node()
                .inner_element()
                .dyn_ref::<HtmlElement>()
                .unwrap()
                .inner_html();
        }

        #[cfg(feature = "ssr")]
        #[allow(unreachable_code)]
        {
            return crate::render_to_string(self.clone());
        }
    }
}

impl Drop for Dom {
    fn drop(&mut self) {
        self.discard()
    }
}

impl Render for Dom {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        parent.append_child(*self)?;
        Ok(())
    }
}
