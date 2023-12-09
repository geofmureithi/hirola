//! ## Mixins
//! Hirola aims to be extensible and follow DRY principles.
//! Here is an example of a mixin
//! ```rust,no_run
//! use hirola::prelude::*;
//! use hirola::dom::Dom;
//! use web_sys::Element;
//! // Mixin that controls tailwind opacity based on a bool signal
//! fn opacity<'a>(signal: &'a Mutable<bool>) -> Box<dyn Fn(&Dom) -> () + 'a> {
//!    let cb = move |dom: &Dom| {
//!        let node = dom.clone();
//!        let element = node.unchecked_into::<Element>();
//!        if signal.get() {
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
//! fn mixin_demo() -> Dom {
//!    let is_shown = Mutable::new(true);
//!    let toggle = is_shown.callback(|show| {
//!         let current = show.get();
//!         *show.lock_mut() = !current;
//!    });
//!    html! {
//!        <div
//!            class="h-screen flex flex-col items-center justify-center transition-all ease-in-out delay-1000">
//!            <div
//!                class="h-64 w-64 block bg-blue-900 rounded-md"
//!                mixin:identity=&opacity(&is_shown)/>
//!            <button
//!                class="bg-gray-200 mt-4 font-bold py-2 px-4 rounded"
//!                on:click=toggle>
//!                "Click Me"
//!            </button>
//!        </div>
//!    }
//! }
//! fn main() {
//!
//! }
//! ```

use crate::generic_node::GenericNode;

pub trait Mixin<Mix, Target> {
    fn mixin(self, node: &Target);
}

/// Unbound mixin in the form of `Fn(&Dom)`
///
/// ## Example
/// ```rust,no_run
/// use hirola::prelude::*;
/// use hirola::dom::Dom;
/// use hirola::dom::mixins::raw_text;
/// 
/// fn counter() -> Dom {
///     html! {
///         <span mixin:identity=&raw_text("Hello Counter!") />
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Identity;

impl<T, Node: GenericNode> Mixin<Identity, Node> for T
where
    T: FnOnce(&Node),
{
    fn mixin(self, node: &Node) {
        (self)(node);
    }
}
