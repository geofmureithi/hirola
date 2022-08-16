use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    str::FromStr,
};

use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

use crate::{
    generic_node::{DomNode, GenericNode},
    prelude::{Mixin, Signal},
};

pub struct Bind<Node, T: 'static>(Signal<T>, PhantomData<Node>);

impl<T: Display + FromStr> Mixin for Bind<HtmlInputElement, T>
where
    <T as FromStr>::Err: Debug,
{
    fn mixin(&self, ns: &str, node: DomNode) {
        assert_eq!(ns, "bind");
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
        node.event("change", handler);
        let input = node.unchecked_into::<HtmlInputElement>();
        input.set_value(&format!("{}", &self.0.get()));
    }
}

pub fn bind_input<T>(s: &Signal<T>) -> Bind<HtmlInputElement, T> {
    Bind(s.clone(), PhantomData)
}
