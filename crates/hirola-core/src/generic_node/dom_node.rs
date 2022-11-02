use std::cell::RefCell;

use ref_cast::RefCast;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{Element, Event, Node, Text};

use crate::generic_node::{EventListener, GenericNode};

/// Rendering backend for the DOM.
///
/// _This API requires the following crate features to be activated: `dom`_
#[derive(Debug, Clone, PartialEq, Eq, RefCast)]
#[repr(transparent)]
pub struct DomNode {
    node: Node,
}

impl DomNode {
    pub fn inner_element(&self) -> Node {
        self.node.clone()
    }
    pub fn unchecked_into<T: JsCast>(self) -> T {
        self.node.unchecked_into()
    }
    // pub fn dyn_into<T: JsCast>(self) -> Result<T, Node> {
    //     self.node.dyn_into()
    // }
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
            node: document().create_document_fragment().into(),
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
        self.node.append_child(&child.node).unwrap();
    }

    fn insert_child_before(&self, new_node: &Self, reference_node: Option<&Self>) {
        self.node
            .insert_before(&new_node.node, reference_node.map(|n| &n.node))
            .unwrap();
    }

    fn remove_child(&self, child: &Self) {
        self.node.remove_child(&child.node).unwrap();
    }

    //fixme: seems like `old` and `new` is backwards?
    fn replace_child(&self, old: &Self, new: &Self) {
        self.node.replace_child(&old.node, &new.node).unwrap();
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

    fn event(&self, name: &str, handler: Box<EventListener>) {
        type EventListener = dyn Fn(Event);

        thread_local! {
            /// A global event listener pool to prevent [`Closure`]s from being deallocated.
            /// TODO: remove events when elements are detached.
            static EVENT_LISTENERS: RefCell<Vec<Closure<EventListener>>> = RefCell::new(Vec::new());
        }

        let closure = Closure::wrap(handler);
        self.node
            .add_event_listener_with_callback(name, closure.as_ref().unchecked_ref())
            .unwrap();

        EVENT_LISTENERS.with(|event_listeners| event_listeners.borrow_mut().push(closure));
    }

    fn update_inner_text(&self, text: &str) {
        self.node
            .dyn_ref::<Text>()
            .unwrap()
            .set_text_content(Some(text));
    }

    
    #[cfg(feature = "transition")]
    fn append_render(&self, child: Box<dyn Fn() -> Box<dyn crate::render::Render<Self>>>) {

        let parent = self.clone();


        let node = crate::prelude::create_effect_initial(crate::cloned!((parent) => move || {
            let node = std::rc::Rc::new(RefCell::new(child().render().node));
            let leaving = std::rc::Rc::new(RefCell::new(false));
            let render = std::rc::Rc::new(RefCell::new(child()));


            let effect = crate::cloned!((node) => move || {
                let leaving = leaving.clone();

                let new_render = child();
                *render.borrow_mut() = new_render;

                //if the current state is leaving, effect will not run the transition and will not replace the node also.
                //the effect will only replace the render
                if !*leaving.borrow() {
                    let node = node.clone();
                    let parent = parent.clone();
                    let render = render.clone();

                    let task = async move {
                        let _ = leaving.replace(true);
                        crate::mixins::leave_transition(node.borrow().clone().unchecked_ref::<Element>().clone()).await;
                        let _ = leaving.replace(false);
                        let new_node = render.borrow().update_node(&parent, &node.borrow());
                        *node.borrow_mut() = new_node.clone();
                        crate::mixins::enter_transition(new_node.unchecked_into::<Element>()).await;
                    };
                    let _fut = wasm_bindgen_futures::spawn_local(task);
                }
            });

            (std::rc::Rc::new(effect), node)
        }));

        parent.append_child(&node.borrow());
    }
    

    #[cfg(not(feature = "transition"))]
    fn append_render(&self, child: Box<dyn Fn() -> Box<dyn crate::render::Render<Self>>>) {
        let parent = self.clone();
        

        let node = crate::prelude::create_effect_initial(crate::cloned!((parent) => move || {
            let node = RefCell::new(child().render().node);


            let effect = crate::cloned!((node) => move || {
                let new_node = child().update_node(&parent, &node.borrow());
                *node.borrow_mut() = new_node;
            });

            (std::rc::Rc::new(effect), node)
        }));

        parent.append_child(&node.borrow());
    }
}
