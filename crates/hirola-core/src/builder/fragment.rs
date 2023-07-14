use crate::{
    render::{Error, Render},
    view::View,
};

// #[derive(Debug)]
pub struct Fragment {
    pub children: Vec<Box<dyn Render>>,
}
impl Fragment {
    pub fn new() -> Fragment {
        Fragment {
            children: Vec::new(),
        }
    }
    pub fn append_child(&mut self, child: impl Render + 'static) {
        self.children.push(Box::new(child))
    }
}

impl Render for Fragment {
    fn render_into(self: Box<Self>, view: &View) -> Result<(), Error> {
        for child in self.children {
            Box::new(child).render_into(view)?;
        }
        Ok(())
    }
}
