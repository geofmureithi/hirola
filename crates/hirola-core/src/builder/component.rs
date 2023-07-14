use crate::{render::Error, view::View};

pub trait Component {
    fn render(self: Box<Self>, view: &View) -> Result<(), Error>;
}
