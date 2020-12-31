use typed_html::dom::VNode;
use typed_html::OutputType;
use web_sys::HtmlElement;
use crate::events::*;
use dominator::DomBuilder;
use crate::component::*;
use std::sync::Arc;
use std::marker::PhantomData;
use std::fmt::{Display, Error, Formatter};
use typed_html::events::EventHandler;
use dominator::traits::StaticEvent;

macro_rules! declare_events {
    ($($name:ident : $type:ty ,)*) => {
        /// Container type for DOM events.
        pub struct Events {
            $(
                pub $name: Option<Box<dyn EventHandler<Node, $type> + Send>>,
            )*
        }

        impl Default for Events {
            fn default() -> Self {
                Events {
                    $(
                        $name: None,
                    )*
                }
            }
        }

        /// Iterate over the defined events on a DOM object.
        #[macro_export]
        macro_rules! for_events {
            ($event:ident in $events:expr => $body:block) => {
                $(
                    if let Some(ref mut $event) = $events.$name $body
                )*
            }
        }
    }
}

// TODO? these are all the "on*" attributes defined in the HTML5 standard, with
// the ones I've been unable to match to Kutuish event types commented out.
//
// This needs review.

declare_events! {
    //abort: ResourceAbortEvent,
    // autocomplete: Event,
    // autocompleteerror: Event,
    blur: Blur,
    // cancel: Event,
    // canplay: Event,
    // canplaythrough: Event,
    change: Change,
    click: Click,    // close: Event,
    contextmenu: ContextMenu,
    // cuechange: Event,
    dblclick: DoubleClick,
    drag: Drag,
    dragend: DragEnd,
    dragenter: DragEnter,
    //dragexit: DragExit,
    dragleave: DragLeave,
    dragover: DragOver,
    dragstart: DragStart,
    drop: Drop,
    // durationchange: Event,
    // emptied: Event,
    // ended: Event,
    //error: ResourceErrorEvent,
    focus: Focus,
    input: Input,
    // invalid: Event,
    keydown: KeyDown,
    //keypress: KeyPress,
    keyup: KeyUp,
    //load: ResourceLoadEvent,
    // loadeddata: Event,
    // loadedmetadata: Event,
    //loadstart: LoadStartEvent,
    mousedown: MouseDown,
    mouseenter: MouseEnter,
    mouseleave: MouseLeave,
    mousemove: MouseMove,
    // mouseout: MouseOut,
    // mouseover: MouseOver,
    mouseup: MouseUp,
    // mousewheel: MouseWheelEvent,
    // pause: Event,
    // play: Event,
    // playing: Event,
    // progress: ProgressEvent,
    // ratechange: Event,
    // reset: Event,
    resize: Resize,
    scroll: Scroll,
    // seeked: Event,
    // seeking: Event,
    // select: Event,
    // show: Event,
    // sort: Event,
    // stalled: Event,
    // submit: Submit,
    // suspend: Event,
    // timeupdate: Event,
    // toggle: Event,
    // volumechange: Event,
    // waiting: Event,
}

impl Display for Events {
    fn fmt(&self, _f: &mut Formatter) -> Result<(), Error> {
        Ok(())
    }
}

impl OutputType for Node
{
    type Events = Events;
    type EventTarget = Node;
    type EventListenerHandle = ();
}

/// Wrapper type for closures as event handlers.
pub struct EFn<F, E>(Option<F>, PhantomData<E>);

impl<F, E> EFn<F, E>
where
    F: FnMut(E) + 'static + Send,
{
    pub fn new(f: F) -> Self {
        EFn(Some(f), PhantomData)
    }
}



struct Callback {
    cb: Box<dyn FnMut(Box<dyn EventHandler<Node, Events> + Send>) + 'static + Send>,
}


impl<F, E> From<F> for Box<dyn EventHandler<Node, E> + Send>
where
    F: FnMut(E) + 'static + Send,
    E: StaticEvent + 'static + Send,
{
    fn from(f: F) -> Self {
        Box::new(EFn::new(f))
    }
}

impl<F, E> EventHandler<Node, E> for EFn<F, E>
where
    F: FnMut(E) + 'static + Send,
    E: StaticEvent + std::marker::Send + 'static,
{
    fn attach(&mut self, target: &mut <Node as OutputType>::EventTarget) {
        let mut handler = self.0.take().unwrap();
        (target.0.cb)(Box::new(handler))

    }

    fn render(&self) -> Option<String> {
        None
    }
}

pub struct Node(Callback);

unsafe impl Send for Node {} 

impl Node {
    pub fn install_handlers(dom: DomBuilder<HtmlElement>, handlers: &mut Events) {
        for_events!(handler in handlers => {
            handler.attach(&mut Node(Callback{
                cb: Box::new(handler)
            }));
        });
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