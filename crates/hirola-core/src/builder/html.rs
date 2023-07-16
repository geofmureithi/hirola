use std::collections::HashMap;

use crate::{generic_node::EventListener, render::Render};

use super::DomBuilder;

pub struct HtmlBuilder {
    pub(super) tag: String,
    pub(super) children: Vec<Box<dyn Render>>,
    pub(super) events: Vec<(&'static str, Box<EventListener>)>,
    pub(super) attributes: HashMap<String, String>,
}

impl HtmlBuilder {
    pub fn new(tag: &str) -> Self {
        Self {
            tag: tag.to_owned(),
            children: vec![],
            events: vec![],
            attributes: HashMap::new(),
        }
    }

    pub fn event(&mut self, name: &'static str, listener: Box<EventListener>) {
        self.events.push((name, listener))
    }

    pub fn append_child(&mut self, child: DomBuilder) {
        self.children.push(Box::new(child))
    }

    pub(crate) fn attribute(&mut self, key: &str, value: String) {
        self.attributes.insert(key.to_owned(), value);
    }
}
