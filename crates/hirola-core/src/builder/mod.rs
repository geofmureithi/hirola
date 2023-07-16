use self::{component::Component, fragment::Fragment, html::HtmlBuilder};
use crate::{
    generic_node::{DomType, EventListener, GenericNode},
    render::{Error, Render},
    view::View,
};
use std::fmt::Display;

pub mod component;
pub mod fragment;
pub mod html;

pub enum DomBuilder {
    Text(String),
    HtmlElement(HtmlBuilder),
    Fragment(Fragment),
    Component(Box<dyn Component>),
}

impl DomBuilder {
    pub fn new() -> DomBuilder {
        DomBuilder::Fragment(Fragment {
            children: Vec::new(),
        })
    }

    pub fn element(tag: &str) -> DomBuilder {
        DomBuilder::HtmlElement(HtmlBuilder::new(tag))
    }

    pub fn event(&mut self, name: &'static str, listener: Box<EventListener>) {
        match self {
            DomBuilder::HtmlElement(element) => element.event(name, listener),
            _ => {
                unreachable!("Events are bound to html elements")
            }
        }
    }

    pub fn attribute(&mut self, key: &str, value: impl Display) {
        match self {
            DomBuilder::HtmlElement(element) => element.attribute(key, value.to_string()),
            _ => {
                unreachable!("Events are bound to html elements")
            }
        }
    }

    // pub fn new_with_node(node: G) -> DomBuilder {
    //     DomBuilder::HtmlElement(HtmlBuilder { tag: (), children: (), events: () })
    // }
}

impl DomBuilder {
    pub fn append_child(&mut self, child: DomBuilder) {
        match self {
            DomBuilder::Text(_) => unreachable!("You cant add children to text"),
            DomBuilder::HtmlElement(inner) => inner.children.push(Box::new(child)),
            DomBuilder::Fragment(frag) => frag.append_child(child),
            DomBuilder::Component(_) => unreachable!("You cant add children to components"),
        }
    }

    pub fn append_render(&mut self, render: impl Render + 'static) {
        let mut fragment = Fragment::new();
        fragment.append_child(render);
        self.append_child(DomBuilder::Fragment(fragment))
    }
}

impl DomBuilder {
    pub fn mount(self, node: &DomType) -> Result<View, Error> {
        let view = View::new_from_node(node);
        Box::new(self).render_into(&view)?;
        Ok(view)
    }
}

impl Render for DomBuilder {
    fn render_into(self: Box<Self>, view: &View) -> Result<(), Error> {
        match *self {
            DomBuilder::Text(text) => Render::render_into(Box::new(text.as_str()), view),
            DomBuilder::HtmlElement(element) => {
                let node = View::new_from_node(&DomType::element(&element.tag));
                for (key, value) in element.attributes {
                    node.attribute(&key, &value);
                }
                for (event, handler) in element.events {
                    node.event(event, handler)
                }
                for child in element.children {
                    Box::new(child).render_into(&node)?;
                }
                view.append_child(node).map_err(Error::DomError)?;
                Ok(())
            }
            DomBuilder::Fragment(frag) => Box::new(frag).render_into(view),
            DomBuilder::Component(c) => {
                c.render(view)?;
                Ok(())
            }
        }
    }
}
