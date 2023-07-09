use std::cell::RefCell;

use ref_cast::RefCast;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{window, Element, Event, Node, Text};

use super::{GenericNode, EventListener};

/// Rendering backend for the DOM.
///
/// _This API requires the following crate features to be activated: `dom`_
#[derive(Debug, Clone, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct DomNode {
    pub node: Node,
}

impl DomNode {
    pub fn inner_element(&self) -> Node {
        self.node.clone()
    }
    pub fn unchecked_into<T: JsCast>(self) -> T {
        self.node.unchecked_into()
    }
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

impl JsCast for DomNode {
    fn instanceof(val: &JsValue) -> bool {
        Node::instanceof(val)
    }

    fn unchecked_from_js(val: JsValue) -> Self {
        DomNode {
            node: Node::unchecked_from_js(val),
        }
    }

    fn unchecked_from_js_ref(val: &JsValue) -> &Self {
        DomNode::ref_cast(Node::unchecked_from_js_ref(val))
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
            Err(e) => window()
                .unwrap()
                .alert_with_message(&format!("{e:?}"))
                .unwrap(),
        }
    }

    fn remove_child(&self, child: &Self) {

        match self.node.remove_child(&child.node) {
            Ok(_) => {}
            Err(e) => window()
                .unwrap()
                .alert_with_message(&format!("{e:?}{:?}", child))
                .unwrap(),
        };
    }

    fn replace_child(&self, old: &Self, new: &Self) {
        match self.node.replace_child(&old.node, &new.node){
            Ok(_) => {}
            Err(e) => window()
                .unwrap()
                .alert_with_message(&format!("{e:?} \n {:?}, {:?}", &old.node, &new.node))
                .unwrap(),
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
}
