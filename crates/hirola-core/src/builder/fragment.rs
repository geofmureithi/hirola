use crate::{
    generic_node::GenericNode,
    render::{Error, Render},
    view::View,
};

use super::ViewBuilder;

// #[derive(Debug)]
pub struct Fragment<G> {
    pub children: Vec<Box<dyn Render<G>>>,
}
impl<G: GenericNode> Fragment<G> {
    pub fn new() -> Fragment<G> {
        Fragment {
            children: Vec::new(),
        }
    }
    pub fn append_child(&mut self, child: impl Render<G> + 'static) {
        self.children.push(Box::new(child))
    }
}

impl<G: GenericNode> Render<G> for Fragment<G> {
    fn render_into(self: Box<Self>, view: &View<G>) -> Result<(), Error> {
        for child in self.children {
            Box::new(child).render_into(view)?;
        }
        Ok(())
    }
}
