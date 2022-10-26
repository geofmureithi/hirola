//! ## Mixins
//! Hirola aims to be extensible and follow DRY principles.
//! Here is an example of a mixin
//! ```rust,no_run
//! use hirola::prelude::*;
//! use web_sys::Element;
//! // Mixin that controls tailwind opacity based on a bool signal
//! fn opacity<'a>(signal: &'a Signal<bool>) -> Box<dyn Fn(DomNode) -> () + 'a> {
//!    let cb = move |node: DomNode| {
//!        let element = node.unchecked_into::<Element>();
//!        if *signal.get() {
//!            element.class_list().add_1("opacity-100").unwrap();
//!            element.class_list().remove_1("opacity-0").unwrap();
//!        } else {
//!            element.class_list().add_1("opacity-0").unwrap();
//!            element.class_list().remove_1("opacity-100").unwrap();
//!        }
//!    };
//!    Box::new(cb)
//! }
//!
//! fn mixin_demo(_app: &HirolaApp) -> Dom {
//!    let is_shown = Signal::new(true);
//!    let toggle = is_shown.mut_callback(|show, _e| !show);
//!    html! {
//!        <div
//!            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
//!            <div
//!                class="h-64 w-64 block bg-blue-900 rounded-md"
//!                mixin:transition=&opacity(&is_shown)/>
//!            <button
//!                class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
//!                on:click=toggle>
//!                "Click Me"
//!            </button>
//!        </div>
//!    }
//! }
//! fn main() {
//!     let window = web_sys::window().unwrap();
//!     let document = window.document().unwrap();
//!     let body = document.body().unwrap();
//!     let app = HirolaApp::new();
//!
//!     app.mount(&body, mixin_demo);
//! }
//! ```
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, HtmlElement, HtmlInputElement};

use crate::{
    callback::MixinError,
    generic_node::{DomNode, GenericNode},
    prelude::{create_effect, Mixin, Signal},
};

/// A mixin that allows adding raw html
/// Note: This is a security risk if the string to be inserted might contain potentially malicious content.
/// sanitize the content before it is inserted.
/// See more: https://developer.mozilla.org/en-US/docs/Web/API/Element/innerHTML
pub fn rhtml<'a>(text: &'a str) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_inner_html(text);
    };
    Box::new(cb)
}

/// A mixin that allows adding nonsignal text
pub fn rtext<'a, D: Display>(text: &'a D) -> Box<dyn Fn(DomNode) + 'a> {
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        element.set_text_content(Some(&format!("{text}")));
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
pub fn text<T: Display>(text: &Signal<T>) -> Box<dyn Fn(DomNode)> {
    let signal = text.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<Element>();
        let signal = signal.clone();
        create_effect(move || {
            let element = element.clone();
            element.set_text_content(Some(&format!("{}", signal.get())));
        });
    };
    Box::new(cb)
}

//show function for using mixin:transition feature
#[cfg(feature = "async")]
pub fn show(shown: &Signal<bool>) -> Box<dyn Fn(DomNode)> {
    let signal = shown.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<HtmlElement>();
        let signal = signal.clone();

        create_effect(move || {
            let res = *signal.get();
            let signal = signal.clone();
            let element = element.clone();
            let task = async move {
                let element = element.clone();
                let style = element.style();
                match res {
                    true => {
                        style.set_property("display", "block").unwrap();
                        enter_transition(element.dyn_into::<Element>().unwrap()).await;
                        
                    },
                    false => {
                        leave_transition(element.dyn_into::<Element>().unwrap()).await;
                        //if transition duration is very long, the `signal` might turned `true` in the transition period.
                        //below is final check whether the `signal` is true of false
                        if !*signal.get() {
                            style.set_property("display", "none").unwrap();
                        }
                    },
                }
            };
            let _fut = wasm_bindgen_futures::spawn_local(task);
        });
    };
    Box::new(cb)
}

/// Mixin that adds text to a dom node
#[cfg(not(feature = "async"))]
pub fn show(shown: &Signal<bool>) -> Box<dyn Fn(DomNode)> {
    let signal = shown.clone();
    let cb = move |node: DomNode| {
        let element = node.unchecked_into::<HtmlElement>();
        let signal = signal.clone();

        create_effect(move || {
            let element = element.clone();
            let style = element.style();
            style
                .set_property("display", {
                    if *signal.get() {
                        "block"
                    } else {
                        "none"
                    }
                })
                .unwrap();
        });
    };
    Box::new(cb)
}

/// Model allows 2-way binding eg between a signal and an input
pub struct Model<Node, T: 'static>(Signal<T>, PhantomData<Node>);

impl<T: Display + FromStr> Mixin for Model<HtmlInputElement, T>
where
    <T as FromStr>::Err: Debug,
{
    fn mixin(&self, ns: &str, node: DomNode) -> Result<(), MixinError> {
        if ns != "model" {
            return Err(MixinError::InvalidNamespace {
                expected: "model".to_string(),
                found: ns.to_string(),
            });
        }
        let input = {
            let node = node.clone();
            node.dyn_into::<HtmlInputElement>()
                .map_err(MixinError::NodeError)?
        };
        let signal = self.0.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let new_value = input.value().parse().unwrap();
            signal.set(new_value);
        });

        node.event("keyup", handler);
        input.set_value(&format!("{}", &self.0.get_untracked()));
        Ok(())
    }
}

/// Two way binding for input and signals
pub mod model {
    use super::*;
    /// Bind a [HtmlInputElement] to a [Signal<T>]
    pub fn input<T>(s: &Signal<T>) -> Model<HtmlInputElement, T> {
        Model(s.clone(), PhantomData)
    }
}


/// Mixin for playing with `transition`
/// 
/// _This mixin requires the following crate features to be activated: `async`_
/// 
/// The implementation is very similar to that in `VueJS` and `Alpine`. 
/// 
/// this mixin takes 2 arguments, a [`Signal`] of String and a boolean, 
/// first argument will be the name of your transition. Reason for wrapping it in a `signal` is to make the 
/// transition dynamic.
/// Second argument is to call the transition when element first appear on the screen 
/// 
/// # Basic Example
/// ```
/// use hirola::prelude::{*, mixins::{show, transition}};
/// 
/// fn your_page(_app: HirolaApp) -> Dom {
///     let state = Signal::new(true);
///     let state_copy = state.clone();
/// 
///     html! {
///         <div>
///             <style>r##"
///             .blue-box {
///                 height: 100px;
///                 width: 100px;
///                 background-color: blue;
///             }
///             
///            .box-enter-active,
///            .box-leave-active {
///                transition: all 0.3s ease;
///            }
///
///            .box-enter-from,
///            .box-leave-to {
///                opacity: 0;
///            }
///
///            .box-enter-to,
///            .box-leave-from {
///                opacity: 1;
///             }
///             "##</style>
///             <div class="blue-box" mixin:show=&show(&state) mixin:transition=&transition(&Signal::new("box".to_string()), true) >
///                 <h1>"Hello from blue box"</h1>
///             </div>
///             <button on:click=move |_| state_copy.set(!*state_copy.get())>"toggle blue box with transition"</button>
///         </div>
///     }
/// } 
/// ```
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub fn transition<'a>(signal: &'a Signal<String>, onenter: bool) -> Box<dyn Fn(DomNode) -> () + 'a> {
    let cb = move |node: DomNode| {
        let signal = signal.clone();
        let node = node.clone();
        let element = node.clone().dyn_into::<Element>().unwrap();

        
        create_effect(move || {
            let signal = signal.get().to_string();
            element.set_attribute("mixintransition", &signal).unwrap();
        });

         
        if onenter {
            let node = node.clone();
            let task = async move {
                let element = node.dyn_ref::<Element>().unwrap();
                enter_transition(element.clone()).await;
            };
            let _fut = wasm_bindgen_futures::spawn_local(task);
        }
        
    };
    Box::new(cb)
}


//transition when element is entering
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn enter_transition(element: Element) {
    let transition = element.get_attribute("mixintransition").unwrap_or_default();
    let element_classes = element.class_list();

    if !transition.is_empty() {
        web_sys::console::log_1(&"enter transiotion2".into());

        //config
        if element_classes.contains(&format!("{}-leave-active", &transition)) {
            let _ = element_classes.remove_1(&format!("{}-leave-from", &transition));
            let _ = element_classes.remove_1(&format!("{}-leave-active", &transition));
            let _ = element_classes.remove_1(&format!("{}-leave-to", &transition));
        }

        if element_classes.contains(&format!("{}-enter-active", &transition)) {
            let _ = element_classes.remove_1(&format!("{}-enter-from", &transition));
            let _ = element_classes.remove_1(&format!("{}-enter-active", &transition));
            let _ = element_classes.remove_1(&format!("{}-enter-to", &transition));
        }

        element_classes.add_1(&format!("{}-enter-from", &transition)).unwrap();
        element_classes.add_1(&format!("{}-enter-active", &transition)).unwrap();
    
        let next_frame_fut = wasm_bindgen_futures::JsFuture::from(next_frame());
        next_frame_fut.await.unwrap();

        
        let _ = element_classes.remove_1(&format!("{}-enter-from", transition)).unwrap();
        element_classes.add_1(&format!("{}-enter-to", transition)).unwrap();

        let wait_for_transition_fut = wasm_bindgen_futures::JsFuture::from(wait_for_transition(element, transition, element_classes));
        wait_for_transition_fut.await.unwrap();
    }

}


//transition when element is leaving
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub async fn leave_transition(element: Element) {
    let transition = element.get_attribute("mixintransition").unwrap_or_default();
    let element_classes = element.class_list();

    if !transition.is_empty() && !element_classes.contains(&format!("{}-leave-active", &transition)) {
        if element_classes.contains(&format!("{}-enter-active", &transition)) {
            let _ = element_classes.remove_1(&format!("{}-enter-from", &transition));
            let _ = element_classes.remove_1(&format!("{}-enter-active", &transition));
            let _ = element_classes.remove_1(&format!("{}-enter-to", &transition));
        }

        element_classes.add_1(&format!("{}-leave-from", &transition)).unwrap();
    
        let next_frame_fut = wasm_bindgen_futures::JsFuture::from(next_frame());
        next_frame_fut.await.unwrap();

        element_classes.add_1(&format!("{}-leave-active", &transition)).unwrap();
        
        let _ = element_classes.remove_1(&format!("{}-leave-from", transition)).unwrap();
        element_classes.add_1(&format!("{}-leave-to", transition)).unwrap();
        
        let wait_for_transition_fut = wasm_bindgen_futures::JsFuture::from(wait_for_transition(element, transition, element_classes));
        wait_for_transition_fut.await.unwrap();
    }
}

//wait for next frame, utils for mixin transition
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
fn next_frame() -> js_sys::Promise {
    js_sys::Promise::new(&mut |resolve, _| {
        let wrapper1 = wasm_bindgen::prelude::Closure::new(move || {
            web_sys::window().unwrap().request_animation_frame(&resolve).unwrap();
        });
        crate::utils::request_animation_frame(std::borrow::Borrow::borrow(&wrapper1));
        wrapper1.forget();
    })
}


//wait for transition to be done
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
fn wait_for_transition(element: Element, transition: String, element_classes: web_sys::DomTokenList) -> js_sys::Promise {
    use std::{rc::Rc, cell::RefCell};

    js_sys::Promise::new(&mut |resolve, _| {
        let element2 = element.clone();
        let element_classes = element_classes.clone();
        let transition = transition.clone();

        let function: Rc<RefCell<Option<wasm_bindgen::prelude::Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
        let function2 = function.clone();

        let transition_function = wasm_bindgen::prelude::Closure::new(Box::new(move || {
            let _ = element_classes.remove_1(&format!("{}-enter-active", &transition));
            let _ = element_classes.remove_1(&format!("{}-enter-to", &transition));
            let _ = element_classes.remove_1(&format!("{}-leave-active", &transition));
            let _ = element_classes.remove_1(&format!("{}-leave-to", &transition));     

            let a = function2.borrow();
            let a = a.as_ref().unwrap();
            element2.remove_event_listener_with_callback("transitionend", a.as_ref().unchecked_ref()).unwrap();

            resolve.call0(&"".into()).unwrap();
        }) as Box<dyn FnMut()>);    

        
        let _a = element.add_event_listener_with_callback("transitionend", transition_function.as_ref().unchecked_ref()).unwrap();
        *function.borrow_mut() = Some(transition_function);
    })
}

