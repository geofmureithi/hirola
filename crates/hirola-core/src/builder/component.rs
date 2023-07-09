use crate::{generic_node::GenericNode, render::Error, view::View};

pub trait Component<G>
where
    G: GenericNode,
{
    fn render(self: Box<Self>, view: &View<G>) -> Result<(), Error>;
}
