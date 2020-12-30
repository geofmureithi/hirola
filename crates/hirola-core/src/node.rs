use typed_html::dom::VNode;
use typed_html::OutputType;
use web_sys::HtmlElement;
use crate::events::*;
use dominator::DomBuilder;
use crate::component::*;
use std::sync::Arc;

impl OutputType for Node
{
    type Events = Events;
    type EventTarget = ();
    type EventListenerHandle = ();
}

pub struct Node;

impl Node {
    pub fn install_handlers(&self, handlers: &mut Events) {
        // for_events!(handler in handlers => {
        //     handler.attach(&mut target);
        // });
    }

    pub fn build<S: State>(state: &Arc<S>, vnode: VNode<'_, Node>) -> DomBuilder<HtmlElement> {
        match vnode {
            VNode::Text(text) => DomBuilder::<HtmlElement>::new_html("span").text(&text),
            VNode::UnsafeText(text) => DomBuilder::<HtmlElement>::new_html("span").text(&text),
            VNode::Element(element) => {
                let mut node = DomBuilder::<HtmlElement>::new_html(element.name);
                for (key, value) in element.attributes {
                    node = node.attribute(&key, &value);
                }

                for child in element.children {
                    let child_node = Node::build(&state, child);
                    node = node.child(&mut child_node.into_dom());
                }
                node
            }
        }
    }
}