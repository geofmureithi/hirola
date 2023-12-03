use html_types::{
    attributes::{Attribute, Value},
    node::{Comment, Element, Node},
    tag::Tag,
};

use super::GenericNode;

#[derive(Clone)]
pub struct SimpleNode<'a>(pub Node<'a>);

impl<'a> GenericNode for SimpleNode<'a> {
    fn element(tag: &str) -> Self {
        SimpleNode(Node::Element(Element::create(
            Tag::try_create(tag).map_err(|_| "invalid tag").unwrap(),
        )))
    }
    fn marker() -> Self {
        SimpleNode(Node::Comment(String::new().into()))
    }

    fn text_node(text: &str) -> Self {
        SimpleNode(Node::Text(String::from(text).into()))
    }

    fn fragment() -> Self {
        todo!()
    }

    fn set_attribute(&self, name: &str, value: &str) {
        match self.0 {
            Node::Text(_) => todo!(),
            Node::Comment(_) => todo!(),
            Node::Element(e) => e.set_attribute(
                Attribute::create(name).map_err(|_| "invalid key").unwrap(),
                Value::create(value).map_err(|_| "invalid tag").unwrap(),
            ),
            Node::Void(_) => todo!(),
        }
    }

    fn append_child(&self, child: &Self) {
        todo!()
    }

    fn insert_child_before(&self, new_node: &Self, reference_node: Option<&Self>) {
        todo!()
    }

    fn remove_child(&self, child: &Self) {
        todo!()
    }

    fn replace_child(&self, old: &Self, new: &Self) {
        todo!()
    }

    fn insert_sibling_before(&self, child: &Self) {
        todo!()
    }

    fn parent_node(&self) -> Option<Self> {
        todo!()
    }

    fn next_sibling(&self) -> Option<Self> {
        todo!()
    }

    fn remove_self(&self) {
        todo!()
    }

    fn update_inner_text(&self, text: &str) {
        todo!()
    }

    fn replace_children_with(&self, node: &Self) {
        todo!()
    }
}
