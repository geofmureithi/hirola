#[macro_use]
extern crate validator_derive;

use std::collections::HashMap;

use hirola_core::{
    prelude::{DomNode, GenericNode, Mixin},
    reactive::Signal,
};
use serde::Serialize;
use serde_json::Value;
use validator::Validate;
use wasm_bindgen::JsCast;
use web_sys::{Element, Event, FormData, HtmlFormElement, HtmlInputElement};

#[derive(Clone)]
pub struct FormHandler<T> {
    inner: Signal<Option<HtmlFormElement>>,
    inputs: Signal<Vec<Element>>,
    value: T,
}

#[derive(Clone)]
pub struct Connect<T> {
    form: FormHandler<T>,
}

impl<T: Serialize + Clone> Mixin for Connect<T> {
    fn mixin(&self, ns: &str, node: DomNode) {
        assert_eq!(ns, "form");
        let form = self.form.clone();
        node.event(
            "submit",
            Box::new(move |e: Event| {
                let element = e.target().unwrap().dyn_into::<HtmlFormElement>();
                let data = FormData::new_with_form(&element.unwrap()).unwrap();
                web_sys::window()
                    .unwrap()
                    .alert_with_message(&format!("{:?}", data.into_serde::<Value>()));
            }),
        );
        let element = node.unchecked_into::<HtmlFormElement>();
        self.form.inner.set(Some(element));
    }
}

#[derive(Clone)]
pub struct Register<T> {
    form: FormHandler<T>,
}

impl<T> Mixin for Register<T> {
    fn mixin(&self, ns: &str, node: DomNode) {
        assert_eq!(ns, "form");
        let element = node.unchecked_into::<Element>();
        self.form.inputs.push(element);
    }
}

impl<T: Clone + Validate> FormHandler<T> {
    fn connect(&self) -> Connect<T> {
        Connect { form: self.clone() }
    }

    fn register(&self) -> Register<T> {
        Register { form: self.clone() }
    }

    pub fn controls(&self) -> (Connect<T>, Register<T>) {
        (
            Connect { form: self.clone() },
            Register { form: self.clone() },
        )
    }

    pub fn submit(&self) {
        let valid = self.value.validate();
    }
}

pub fn register<T: Clone>(form: &FormHandler<T>) -> Register<T> {
    Register { form: form.clone() }
}
