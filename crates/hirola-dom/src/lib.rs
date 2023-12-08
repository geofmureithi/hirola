pub mod app;
pub mod mixins;
pub mod node_ref;

use core::fmt;
use discard::{DiscardOnDrop, Discard};
use hirola_core::prelude::cancelable_future;
use hirola_core::render::Render;
use hirola_core::{
    generic_node::{EventListener, GenericNode},
    prelude::CancelableFutureHandle,
    render::Error,
    BoxedLocal,
};
use std::rc::Rc;
use std::{cell::RefCell, future::Future};
use wasm_bindgen::{prelude::*, JsCast};
pub use web_sys::Event;
use web_sys::{Element, Node, Text};

pub enum DomSideEffect {
    UnMounted(BoxedLocal<()>),
    Mounted(CancelableFutureHandle),
}

/// Rendering backend for the DOM.
///
/// The `DomNode` struct represents a node in the Document Object Model (DOM) and serves as the
/// rendering backend for the frontend application. It allows interacting with DOM nodes directly
/// and provides utility methods for type conversion and cloning.
///
/// _This API requires the following crate features to be activated: `dom`_
///
#[derive(Clone)]
pub struct Dom {
    pub node: Node,
    pub side_effects: Rc<RefCell<Vec<DomSideEffect>>>,
    event_handlers: Rc<RefCell<Vec<Closure<dyn Fn(Event)>>>>,
    children: RefCell<Vec<Dom>>,
}

impl fmt::Debug for Dom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Dom")
            .field("node", &self.node)
            .field("side_effects", &self.side_effects.borrow().len())
            .field("event_handlers", &self.event_handlers.borrow())
            .field("children", &self.children.borrow())
            .finish()
    }
}

impl PartialEq for Dom {
    fn eq(&self, other: &Self) -> bool {
        self.node == other.node
    }
}

impl Eq for Dom {}

impl Default for Dom {
    fn default() -> Self {
        Dom {
            node: document().create_document_fragment().dyn_into().unwrap(),
            side_effects: Default::default(),
            event_handlers: Default::default(),
            children: Default::default(),
        }
    }
}

impl Dom {
    pub fn inner_html(&self) -> String {
        {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let element = document.create_element("div").unwrap();
            crate::render_to(self.clone(), &element.clone().try_into().unwrap()).unwrap();
            return element.inner_html();
        }
    }

    pub fn new_from_node(node: &Node) -> Self {
        Dom {
            node: node.clone(),
            ..Default::default()
        }
    }

    pub fn discard(&mut self) {
        let _cleanup: Vec<()> = self
            .event_handlers
            .take()
            .into_iter()
            .map(|c| c.forget())
            .collect();
        let _cleanup: Vec<()> = self
            .side_effects
            .take()
            .into_iter()
            .map(|e| match e {
                DomSideEffect::Mounted(e) => e.discard(),
                DomSideEffect::UnMounted(_) => {
                    log::warn!("Dropping a side effect that was not mounted")
                }
            })
            .collect();
    }
}

impl Dom {
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

impl AsRef<JsValue> for Dom {
    fn as_ref(&self) -> &JsValue {
        self.node.as_ref()
    }
}

impl From<Dom> for JsValue {
    fn from(node: Dom) -> Self {
        node.node.into()
    }
}

fn document() -> web_sys::Document {
    web_sys::window().unwrap().document().unwrap()
}

impl GenericNode for Dom {
    fn element(tag: &str) -> Self {
        Dom {
            node: document().create_element(tag).unwrap().dyn_into().unwrap(),
            ..Default::default()
        }
    }

    fn text_node(text: &str) -> Self {
        Dom {
            node: document().create_text_node(text).into(),
            ..Default::default()
        }
    }

    fn fragment() -> Self {
        Dom {
            node: document().create_document_fragment().dyn_into().unwrap(),
            ..Default::default()
        }
    }

    fn marker() -> Self {
        Dom {
            node: document().create_comment("").into(),
            ..Default::default()
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
            _ => {
                self.children.borrow_mut().push(child.clone());
            }
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
        let n =self.node.parent_node().unwrap();
        Some(Self {
            node: n,
            ..Default::default()
        })
    }

    fn next_sibling(&self) -> Option<Self> {
        self.node.next_sibling().map(|node| Self {
            node,
            ..Default::default()
        })
    }

    fn remove_self(&self) {
        self.node.unchecked_ref::<Element>().remove();
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

    fn effect(&self, future: impl std::future::Future<Output = ()> + 'static) {
        self.side_effects
            .borrow_mut()
            .push(DomSideEffect::Mounted(DiscardOnDrop::leak(spawn(future))));
    }

    fn children(&self) -> RefCell<Vec<Self>> {
        self.children.clone()
    }
}

/// Render a [`Dom`] into the DOM.
/// Alias for [`render_to`] with `parent` being the `<body>` tag.
pub fn render(dom: Dom) -> Result<Dom, Error> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    render_to(dom, &document.body().unwrap())
}

/// Render a [`Dom`] under a `parent` node.
/// For rendering under the `<body>` tag, use [`render()`] instead.

pub fn render_to(dom: Dom, parent: &web_sys::Node) -> Result<Dom, Error> {
    let parent = Dom {
        node: parent.clone(),
        ..Default::default()
    };
    parent.append_child(&dom);
    Ok(parent)
}

impl EventListener for Dom {
    type Handler = Box<dyn Fn(web_sys::Event)>;
    fn event(&self, name: &str, handler: Self::Handler) {
        let closure = Closure::wrap(handler);
        self.node
            .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
            .unwrap();
        self.event_handlers.borrow_mut().push(closure);
    }
}

#[inline]
pub fn spawn<F>(future: F) -> DiscardOnDrop<CancelableFutureHandle>
where
    F: Future<Output = ()> + 'static,
{
    let (handle, future) = cancelable_future(future, || ());

    wasm_bindgen_futures::spawn_local(future);

    handle
}

impl Render<Dom> for Dom {
    fn render_into(self: Box<Self>, parent: &Dom) -> Result<(), Error> {
        parent.append_child(&self);
        Ok(())
    }
}


pub mod dom_test_utils {
    use wasm_bindgen::{prelude::Closure, JsCast};

    pub fn next_tick_with<N: Clone + 'static>(with: &N, f: impl Fn(&N) -> () + 'static) {
        let with = with.clone();
        let f: Box<dyn Fn() -> ()> = Box::new(move || f(&with));
        let a = Closure::<dyn Fn()>::new(f);
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(a.as_ref().unchecked_ref())
            .unwrap();
    }

    pub fn next_tick<F: Fn() + 'static>(f: F) {
        let a = Closure::<dyn Fn()>::new(move || f());
        web_sys::window()
            .unwrap()
            .set_timeout_with_callback(a.as_ref().unchecked_ref())
            .unwrap();
    }
}
