use super::{EventListener, GenericNode};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event, Node, Text};

/// Rendering backend for the DOM.
///
/// The `DomNode` struct represents a node in the Document Object Model (DOM) and serves as the
/// rendering backend for the frontend application. It allows interacting with DOM nodes directly
/// and provides utility methods for type conversion and cloning.
///
/// _This API requires the following crate features to be activated: `dom`_
///
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct DomNode {
    pub node: Node,
}

impl DomNode {
    /// Retrieves the inner DOM node contained within the `DomNode`.
    ///
    /// # Returns
    ///
    /// The underlying DOM node represented by this `DomNode`.
    pub fn inner_element(&self) -> Node {
        self.node.clone()
    }
    /// Converts the `DomNode` into a specified type using unchecked casting.
    ///
    /// This method allows converting the `DomNode` into a specific type, without performing a
    /// runtime type check. It can be used when you are confident about the type of the DOM node,
    /// and it avoids the overhead of dynamic type checking.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to convert the `DomNode` into. It should implement the `JsCast`
    ///         trait, which provides the unchecked casting functionality.
    ///
    /// # Returns
    ///
    /// The converted `DomNode` as the target type `T`.
    pub fn unchecked_into<T: JsCast>(self) -> T {
        self.node.unchecked_into()
    }
    /// Attempts to dynamically cast the `DomNode` into a specified type.
    ///
    /// This method performs a runtime type check to determine if the `DomNode` can be converted
    /// into the desired type. If the conversion succeeds, it returns the converted value;
    /// otherwise, it returns an error containing the original `DomNode`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The target type to cast the `DomNode` into. It should implement the `JsCast`
    ///         trait, which provides the dynamic type casting functionality.
    ///
    /// # Returns
    ///
    /// - `Ok(T)` if the `DomNode` was successfully cast into the target type `T`.
    /// - `Err(Node)` if the `DomNode` could not be cast into the target type `T`.

    pub fn dyn_into<T: JsCast>(self) -> Result<T, Node> {
        self.node.dyn_into()
    }
}

impl AsRef<JsValue> for DomNode {
    fn as_ref(&self) -> &JsValue {
        self.node.as_ref()
    }
}

impl From<DomNode> for JsValue {
    fn from(node: DomNode) -> Self {
        node.node.into()
    }
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

impl GenericNode for DomNode {
    fn element(tag: &str) -> Self {
        DomNode {
            node: document().create_element(tag).unwrap().dyn_into().unwrap(),
        }
    }

    fn text_node(text: &str) -> Self {
        DomNode {
            node: document().create_text_node(text).into(),
        }
    }

    fn fragment() -> Self {
        DomNode {
            node: document().create_document_fragment().dyn_into().unwrap(),
        }
    }

    fn marker() -> Self {
        DomNode {
            node: document().create_comment("").into(),
        }
    }

    fn set_attribute(&self, name: &str, value: &str) {
        self.node
            .unchecked_ref::<Element>()
            .set_attribute(name, value)
            .unwrap();
    }

    fn append_child(&self, child: &Self) {
        match self.node.append_child(&child.node) {
            Err(e) => log::warn!("Could not append child: {e:?}"),
            _ => {}
        }
    }

    fn insert_child_before(&self, new_node: &Self, reference_node: Option<&Self>) {
        match self
            .node
            .insert_before(&new_node.node, reference_node.map(|n| &n.node))
        {
            Ok(_) => {}
            Err(e) => log::warn!("Failed to insert child: {e:?}"),
        }
    }

    fn remove_child(&self, child: &Self) {
        match self.node.remove_child(&child.node) {
            Ok(_) => {}
            Err(e) => log::warn!("Failed to remove child: {e:?}"),
        };
    }

    fn replace_child(&self, old: &Self, new: &Self) {
        match self.node.replace_child(&old.node, &new.node) {
            Ok(_) => {}
            Err(e) => log::warn!("Failed to replace child: {e:?}"),
        };
    }

    fn insert_sibling_before(&self, child: &Self) {
        self.node
            .unchecked_ref::<Element>()
            .before_with_node_1(&child.node)
            .unwrap();
    }

    fn parent_node(&self) -> Option<Self> {
        self.node.parent_node().map(|node| Self { node })
    }

    fn next_sibling(&self) -> Option<Self> {
        self.node.next_sibling().map(|node| Self { node })
    }

    fn remove_self(&self) {
        self.node.unchecked_ref::<Element>().remove();
    }

    fn event(&self, name: &str, handler: Box<EventListener>) -> Option<Closure<dyn Fn(Event)>> {
        let closure = Closure::wrap(handler);
        self.node
            .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
            .unwrap();
        Some(closure)
    }

    fn update_inner_text(&self, text: &str) {
        self.node
            .dyn_ref::<Text>()
            .unwrap()
            .set_text_content(Some(text));
    }
    fn replace_children_with(&self, node: &Self) {
        let element = self.node.unchecked_ref::<Element>();
        element.replace_children_with_node_1(&node.inner_element())
    }
}
