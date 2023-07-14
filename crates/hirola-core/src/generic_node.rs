#[cfg(feature = "dom")]
pub mod dom_node;
#[cfg(feature = "ssr")]
pub mod ssr_node;

#[cfg(feature = "dom")]
pub use dom_node::*;
#[cfg(feature = "ssr")]
pub use ssr_node::*;
use wasm_bindgen::prelude::Closure;

use std::fmt;

use web_sys::Event;

pub type EventListener = dyn Fn(Event);

#[cfg(feature = "dom")]
pub type DomType = dom_node::DomNode;

#[cfg(not(feature = "dom"))]
pub type DomType = ssr_node::SsrNode;

pub trait GenericNode: fmt::Debug + Clone + PartialEq + std::cmp::Eq + 'static {
    /// Create a new element node.
    fn element(tag: &str) -> Self;

    /// Create a new text node.
    fn text_node(text: &str) -> Self;

    /// Create a new fragment (list of nodes). A fragment is not necessarily wrapped around by an element.
    fn fragment() -> Self;

    /// Create a marker (dummy) node. For [`DomNode`], this is implemented by creating an empty comment node.
    /// This is used, for example, in [`Keyed`] and [`Indexed`] for scenarios where you want to push a new item to the
    /// end of the list. If the list is empty, a dummy node is needed to store the position of the component.
    fn marker() -> Self;

    /// Sets an attribute on a node.
    fn set_attribute(&self, name: &str, value: &str);

    /// Appends a child to the node's children.
    fn append_child(&self, child: &Self);

    /// Insert a new child node to this node's children. If `reference_node` is `Some`, the child will be inserted
    /// before the reference node. Else if `None`, the child will be inserted at the end.
    fn insert_child_before(&self, new_node: &Self, reference_node: Option<&Self>);

    /// Remove a child node from this node's children.
    fn remove_child(&self, child: &Self);

    /// Replace a child node from this node's children with a new child node.
    fn replace_child(&self, old: &Self, new: &Self);

    /// Insert a new node before this node.
    fn insert_sibling_before(&self, child: &Self);

    /// Returns the parent node, or `None` if detached.
    fn parent_node(&self) -> Option<Self>;

    /// Returns the next sibling, or `None` if this node is the last sibling.
    fn next_sibling(&self) -> Option<Self>;

    /// Remove this node from the tree.
    fn remove_self(&self);

    /// Add a [`EventListener`] to the event `name`.
    fn event(&self, _name: &str, _handler: Box<EventListener>) -> Option<Closure<dyn Fn(Event)>> {
        None
    }

    /// Update inner text of the node. If the node has elements, all the elements are replaced with a new text node.
    fn update_inner_text(&self, text: &str);

    /// Replace all the children in a node with a new node
    fn replace_children_with(&self, node: &Self);
}
