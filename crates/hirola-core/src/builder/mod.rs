use crate::{
    generic_node::{GenericNode, EventListener},
    render::{Error, Render},
    view::View,
};

use self::{component::Component, fragment::Fragment, html::HtmlBuilder};

pub mod component;
pub mod fragment;
pub mod html;

pub enum ViewBuilder<G> {
    Text(String),
    HtmlElement(HtmlBuilder<G>),
    Fragment(Fragment<G>),
    Component(Box<dyn Component<G>>),
}

impl<G: GenericNode> ViewBuilder<G> {
    pub fn new() -> ViewBuilder<G> {
        ViewBuilder::Fragment(Fragment {
            children: Vec::new(),
        })
    }

    pub fn element(tag: &str) -> ViewBuilder<G> {
        ViewBuilder::HtmlElement(HtmlBuilder::new(tag))
    }

    pub fn event(&mut self, name: &'static str, listener: Box<EventListener>) {
        match self {
            ViewBuilder::HtmlElement(element) => element.event(name, listener),
            _ => {
                unreachable!("Events are bound to html elements")
            }
        }
    }

    // pub fn new_with_node(node: G) -> ViewBuilder<G> {
    //     ViewBuilder::HtmlElement(HtmlBuilder { tag: (), children: (), events: () })
    // }
}

impl<G: 'static + GenericNode> ViewBuilder<G> {
    pub fn append_child(&mut self, child: ViewBuilder<G>) {
        match self {
            ViewBuilder::Text(_) => unreachable!("You cant add children to text"),
            ViewBuilder::HtmlElement(inner) => inner.children.push(Box::new(child)),
            ViewBuilder::Fragment(frag) => frag.append_child(child),
            ViewBuilder::Component(_) => unreachable!("You cant add children to components"),
        }
    }

    pub fn append_render(&mut self, render: impl Render<G> + 'static) {
        let mut fragment = Fragment::new();
        fragment.append_child(render);
        self.append_child(ViewBuilder::Fragment(fragment))
    }
}

impl<G: GenericNode> ViewBuilder<G> {
    pub fn mount(self, node: &G) -> Result<View<G>, Error> {
        let view = View::new_from_node(node);
        Box::new(self).render_into(&view)?;
        Ok(view)
    }
}

impl<G: GenericNode> Render<G> for ViewBuilder<G> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        match *self {
            ViewBuilder::Text(text) => Render::<G>::render_into(Box::new(text.as_str()), view),
            ViewBuilder::HtmlElement(element) => {
                let node = View::new_from_node(&G::element(&element.tag));
                for (event, handler) in element.events {
                    node.event(event, handler)
                }
                for child in element.children {
                    Box::new(child).render_into(&node)?;
                }
                view.append_child(node).map_err(Error::DomError)?;
                Ok(())
            }
            ViewBuilder::Fragment(frag) => Box::new(frag).render_into(view),
            ViewBuilder::Component(c) => {
                c.render(view)?;
                Ok(())
            }
        }
    }
}
