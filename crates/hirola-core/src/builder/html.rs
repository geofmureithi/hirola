use crate::{
    generic_node::{EventListener, GenericNode},
    render::Render,
};

use super::{fragment::Fragment, ViewBuilder};

pub struct HtmlBuilder<G> {
    pub(super) tag: String,
    pub(super) children: Vec<Box<dyn Render<G>>>,
    pub(super) events: Vec<(&'static str, Box<EventListener>)>,
}

impl<G: 'static + GenericNode> HtmlBuilder<G> {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            children: vec![],
            events: vec![],
        }
    }

    pub fn event(&mut self, name: &'static str, listener: Box<EventListener>) {
        self.events.push((name, listener))
    }

    pub fn append_child(&mut self, child: ViewBuilder<G>) {
        self.children.push(Box::new(child))
    }
}
