use std::fmt::Display;

use futures_signals::signal::{DedupeMap, Mutable, Signal, SignalExt};

use crate::prelude::GenericNode;

/// Trait for defining side effects.
///
/// The `SideEffect` trait allows defining asynchronous side effects that are executed as futures.
/// Implementations of this trait should represent tasks that need to be performed concurrently
/// with the rendering process, such as making HTTP requests, updating global state, or scheduling
/// timers.
///
/// When used in conjunction with the `Dom`, side effects can be attached to specific DOM nodes and
/// executed during the rendering process, ensuring proper handling of asynchronous operations
/// within the frontend application.
///
/// # Example
///
/// ```
/// use std::future::ready;
/// use hirola::prelude::*;
/// // Define a custom side effect that executes asynchronously
/// struct CustomSideEffect;
///
/// impl SideEffect for CustomSideEffect {
///     fn effect(self) -> BoxedLocal<()> {
///         // Perform some asynchronous task and return a future that represents its completion
///         Box::pin(ready(()))
///     }
/// }
/// ```
pub trait SideEffect<Attr: EffectAttribute<Handler = Self>, Effect, Node> {
    fn effect(&self, node: &Node, attr: Attr, effect: Effect);
}

pub trait EffectAttribute {
    type Handler;
    fn read_as_attr(&self) -> String;
}

pub struct DefaultAttributeEffect;

pub struct DefaultAttrStr(pub &'static str);

impl EffectAttribute for DefaultAttrStr {
    type Handler = DefaultAttributeEffect;
    fn read_as_attr(&self) -> String {
        self.0.to_owned()
    }
}

macro_rules! impl_simple_effect {
    ($effect_type:ty) => {
        impl<Node: GenericNode> SideEffect<DefaultAttrStr, $effect_type, Node>
            for DefaultAttributeEffect
        {
            fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: $effect_type) {
                node.set_attribute(attr.0, &effect.to_string())
            }
        }
    };
}

// Usage of the macro for different types
impl_simple_effect!(&str);
impl_simple_effect!(String);
impl_simple_effect!(&String);
impl_simple_effect!(bool);
impl_simple_effect!(usize);
impl_simple_effect!(i8);
impl_simple_effect!(i64);
impl_simple_effect!(i32);
impl_simple_effect!(i128);
impl_simple_effect!(u8);
impl_simple_effect!(u16);
impl_simple_effect!(u32);
impl_simple_effect!(u64);
impl_simple_effect!(u128);

// macro_rules! impl_signal_effect {
//     ($effect_type:ty) => {
//         impl<Node: GenericNode, A: Display + 'static + Clone + PartialEq>
//             SideEffect<DefaultAttrStr, $effect_type, Node> for DefaultAttributeEffect
//         {
//             fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: $effect_type) {
//                 let dom = node.clone();
//                 let future = SignalExt::dedupe_map(effect, move |value| {
//                     GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
//                 })
//                 .to_future();
//                 node.effect(future);
//             }
//         }
//     };
// }

impl<Node: GenericNode, A: Display + 'static + Clone + PartialEq>
    SideEffect<DefaultAttrStr, Mutable<A>, Node> for DefaultAttributeEffect
{
    fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: Mutable<A>) {
        let dom = node.clone();
        let future = SignalExt::dedupe_map(effect.signal_cloned(), move |value| {
            GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
        })
        .to_future();
        node.effect(future);
    }
}

impl<
        Node: GenericNode,
        F: FnMut(&mut <S as Signal>::Item) -> A + 'static,
        S: Signal + 'static,
        A: Display + 'static + Clone + PartialEq,
    > SideEffect<DefaultAttrStr, DedupeMap<S, F>, Node> for DefaultAttributeEffect
where
    <S as Signal>::Item: PartialEq,
{
    fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: DedupeMap<S, F>) {
        let dom = node.clone();
        let future = SignalExt::dedupe_map(effect, move |value| {
            GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
        })
        .to_future();
        node.effect(future);
    }
}

use futures_signals::signal::Dedupe;

impl<
        Node: GenericNode,
        S: Signal<Item = A> + 'static,
        A: Display + 'static + Copy + PartialEq,
    > SideEffect<DefaultAttrStr, Dedupe<S>, Node> for DefaultAttributeEffect
where
    <S as Signal>::Item: PartialEq,
{
    fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: Dedupe<S>) {
        let dom = node.clone();
        let future = SignalExt::dedupe_map(effect, move |value| {
            GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
        })
        .to_future();
        node.effect(future);
    }
}

use futures_signals::signal::DedupeCloned;

impl<
        Node: GenericNode,
        S: Signal<Item = A> + 'static,
        A: Display + 'static + Clone + PartialEq,
    > SideEffect<DefaultAttrStr, DedupeCloned<S>, Node> for DefaultAttributeEffect
where
    <S as Signal>::Item: PartialEq,
{
    fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: DedupeCloned<S>) {
        let dom = node.clone();
        let future = SignalExt::dedupe_map(effect, move |value| {
            GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
        })
        .to_future();
        node.effect(future);
    }
}


use futures_signals::signal::Map;

impl<
        Node: GenericNode,
        F: FnMut(<S as Signal>::Item) -> A + 'static,
        S: Signal + 'static,
        A: Display + 'static + Clone + PartialEq,
    > SideEffect<DefaultAttrStr, Map<S, F>, Node> for DefaultAttributeEffect
where
    <S as Signal>::Item: PartialEq,
{
    fn effect(&self, node: &Node, attr: DefaultAttrStr, effect: Map<S, F>) {
        let dom = node.clone();
        let future = SignalExt::map(effect, move |value| {
            GenericNode::set_attribute(&dom, &attr.0, &value.to_string());
        })
        .to_future();
        node.effect(future);
    }
}
