pub mod prelude {
    use super::*;
    pub use attr_bind::*;
    pub use attr_mixin::*;
    pub use attr_on::*;
    pub use attr_use::*;
}

pub mod attr_use {
    use std::fmt::Display;

    use hirola_core::{
        effect::EffectAttribute,
        prelude::{signal::SignalExt, GenericNode, SideEffect},
    };
    pub struct UseEffect;

    pub struct Future;

    impl EffectAttribute for Future {
        type Handler = UseEffect;
        fn read_as_attr(&self) -> String {
            "future".to_owned()
        }
    }
    impl<F: std::future::Future<Output = ()> + 'static, N: GenericNode> SideEffect<Future, F, N>
        for UseEffect
    {
        fn effect(&self, node: &N, _marker: Future, future: F) {
            node.effect(future)
        }
    }

    pub struct SignalAttr<S: hirola_core::prelude::signal::Signal>(String, S);

    pub fn attr_signal<S: hirola_core::prelude::signal::Signal>(
        attr: &str,
        signal: S,
    ) -> SignalAttr<S> {
        SignalAttr(attr.to_owned(), signal)
    }

    pub struct Signal;

    impl EffectAttribute for Signal {
        type Handler = UseEffect;
        fn read_as_attr(&self) -> String {
            "signal".to_owned()
        }
    }

    impl<
            S: hirola_core::prelude::signal::Signal<Item = D> + 'static,
            D: Display + PartialEq,
            N: GenericNode,
        > SideEffect<Signal, SignalAttr<S>, N> for UseEffect
    {
        fn effect(&self, dom: &N, _: Signal, signal: SignalAttr<S>) {
            let node = dom.clone();
            let attr = signal.0;
            let future = SignalExt::dedupe_map(signal.1, move |value| {
                GenericNode::set_attribute(&node, &attr, &value.to_string());
            })
            .to_future();

            dom.effect(future);
        }
    }
}

pub mod attr_on {
    pub struct OnEffect;

    use hirola_core::{
        effect::{EffectAttribute, SideEffect},
        generic_node::{EventListener, GenericNode},
    };
    use web_sys::Event;

    pub use crate::types::DomEvent::*;
    impl<
            D: EffectAttribute<Handler = OnEffect>,
            F: Fn(Event) + 'static,
            N: GenericNode + EventListener<F>,
        > SideEffect<D, F, N> for OnEffect
    {
        fn effect(&self, node: &N, attr: D, effect: F) {
            node.event(&attr.read_as_attr(), effect)
        }
    }
}

pub mod attr_bind {
    pub struct Value;
    use std::{
        fmt::{Debug, Display},
        str::FromStr,
    };

    use hirola_core::{
        effect::{EffectAttribute, SideEffect},
        generic_node::{EventListener, GenericNode, NodeReference},
        prelude::{signal::SignalExt, Mutable},
    };
    use wasm_bindgen::JsCast;
    use web_sys::{Event, HtmlInputElement};

    use crate::{node_ref::NodeRef, Dom};
    impl EffectAttribute for Value {
        type Handler = BindEffect;
        fn read_as_attr(&self) -> String {
            "value".to_owned()
        }
    }

    pub struct BindEffect;

    impl EffectAttribute for Ref {
        type Handler = BindEffect;
        fn read_as_attr(&self) -> String {
            "ref".to_owned()
        }
    }

    pub struct Ref;

    impl SideEffect<Ref, NodeRef, Dom> for BindEffect {
        fn effect(&self, node: &Dom, _attr: Ref, value: NodeRef) {
            NodeReference::set(&value, node.clone());
        }
    }

    impl<
            T: Clone + Display + PartialEq + 'static + FromStr,
            D: EffectAttribute<Handler = BindEffect>,
            N: GenericNode + EventListener<Box<dyn Fn(Event)>>,
        > SideEffect<D, &Mutable<T>, N> for BindEffect
    where
        <T as FromStr>::Err: Debug,
    {
        fn effect(&self, node: &N, attr: D, value: &Mutable<T>) {
            let attr = attr.read_as_attr().to_owned();
            let dom = node.clone();
            let future = SignalExt::dedupe_map(value.signal_cloned(), move |value| {
                GenericNode::set_attribute(&dom, &attr, &format!("{}", value));
            })
            .to_future();
            let value = value.clone();
            let handler = move |e: Event| {
                let input = e
                    .current_target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                let new_value = input.value();
                value.set(new_value.parse().unwrap());
            };
            node.event("input", Box::new(handler));
            node.effect(future);
        }
    }
}

pub mod attr_mixin {
    pub struct XEffect;

    use hirola_core::effect::{EffectAttribute, SideEffect};
    pub use XEffect as MixinEffect;

    use crate::Dom;

    pub use crate::mixins::*;

    impl<A: EffectAttribute<Handler = XEffect>> SideEffect<A, Box<dyn FnOnce(&Dom)>, Dom> for XEffect {
        fn effect(&self, node: &Dom, _: A, effect: Box<dyn FnOnce(&Dom)>) {
            effect(node);
        }
    }
}
