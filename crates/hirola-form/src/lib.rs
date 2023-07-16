use hirola_core::prelude::{
    signal::{Mutable, MutableSignalRef, ReadOnlyMutable, SignalExt},
    Dom, DomType, GenericNode, Mixin, NodeRef,
};
use json_dotpath::DotPaths;
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, marker::PhantomData, rc::Rc};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};

#[derive(Debug)]
pub enum Error {
    Json(serde_json::Error),
    InvalidSetter(json_dotpath::Error),
    InvalidGetter(json_dotpath::Error),
}

pub trait Validate: Sized {
    type Error;
    fn validate(&self) -> Result<(), Self::Error>;
    fn errors(&self) -> HashMap<&'static str, String>;
}

pub struct Form;

/// Form plugin for hirola
#[derive(Clone, Debug)]
pub struct FormHandler<T: 'static> {
    node_ref: NodeRef,
    value: Mutable<T>,
}

impl<T: Serialize + DeserializeOwned + Clone> FormHandler<T> {
    /// Build a new reactive form
    pub fn new(value: T) -> Self {
        Self {
            node_ref: NodeRef::new(),
            value: Mutable::new(value),
        }
    }

    /// Get the immutable handle for form value
    pub fn handle(&self) -> ReadOnlyMutable<T> {
        (&self.value).read_only()
    }

    /// Update a specific field using the dot notation.
    /// Eg you can update person.email
    pub fn update_field<S: Serialize>(&self, name: &str, value: S) -> Result<(), Error> {
        let current_value = self.value.clone();
        let mut json = serde_json::to_value(&current_value).map_err(Error::Json)?;
        json.dot_set(&name, value).map_err(Error::InvalidSetter)?;
        let ser: T = serde_json::from_value(json).map_err(Error::Json)?;
        current_value.set(ser);
        Ok(())
    }

    /// Get and cast a field value
    pub fn get_value_by_field<S: DeserializeOwned>(&self, name: &str) -> Result<Option<S>, Error> {
        let current_value = self.value.clone();
        let json = serde_json::to_value(&current_value).map_err(Error::Json)?;
        json.dot_get(name).map_err(Error::InvalidGetter)
    }

    // Get form value
    pub fn get_value(&self) -> T {
        self.value.get_cloned()
    }
}

/// Allows you to register form elements
#[derive(Clone)]
pub struct Register<T: 'static, E> {
    form: FormHandler<T>,
    element_type: PhantomData<E>,
}

impl<T: Serialize + DeserializeOwned + Clone> Mixin<Form> for Register<T, HtmlInputElement> {
    fn mixin(&self, dom: &Dom) {
        let form = self.form.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let name = input.name();

            let new_value = input.value();
            form.update_field(&name, new_value).unwrap();
        });
        dom.event("input", handler);
        let input = {
            let node = dom.node().clone();
            node.dyn_into::<HtmlInputElement>().unwrap()
        };
        let name = input.name();
        let value: String = self.form.get_value_by_field(&name).unwrap().unwrap();
        dom.node().set_attribute("value", &value);
    }
}

impl<T: Serialize + DeserializeOwned + Clone> Mixin<Form> for Register<T, HtmlSelectElement> {
    fn mixin(&self, node: &Dom) {
        let form = self.form.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let name = input.name();

            let new_value = input.value();
            form.update_field(&name, new_value).unwrap();
        });
        node.event("change", handler);
    }
}

/// Allows connecting non-standard elements(eg components) to forms
#[derive(Clone, Debug)]
pub struct Bind<B: 'static, F: 'static>(&'static str, FormHandler<F>, PhantomData<B>);

impl<B: Serialize + DeserializeOwned, F: Serialize + DeserializeOwned + Clone> Bind<B, F> {
    /// Manually set the value of the bound field
    pub fn set_value(&self, value: B) -> Result<(), Error> {
        self.1.update_field(self.0, value)
    }

    /// Get value of bound field
    pub fn get_value(&self) -> MutableSignalRef<F, impl FnMut(&F) -> B> {
        let current_value = self.1.value.clone();
        let name = self.0;
        fn read_inner_value<F, B>(value: &F, name: &str) -> B
        where
            B: Serialize + DeserializeOwned,
            F: Serialize + DeserializeOwned,
        {
            let json = serde_json::to_value(value).unwrap();
            json.dot_get(name).unwrap().unwrap()
        }
        current_value.signal_ref(|value| read_inner_value::<F, B>(&value, name))
    }
}

impl<T: Validate + Clone> FormHandler<T> {
    /// Perform validation
    pub fn validate(&self) -> Result<(), <T as Validate>::Error> {
        self.value.get_cloned().validate()
    }

    /// Get error specific field
    pub fn error_for(&self, name: &'static str) -> MutableSignalRef<T, impl FnMut(&T) -> String> {
        self.value.signal_ref(|value: &T| {
            let errors = value.errors();
            if let Some(err) = errors.get(name) {
                return err.clone();
            }
            "String::new()".to_owned()
        })
    }
}

impl<T: Clone + Serialize + DeserializeOwned> FormHandler<T> {
    /// Create a form binding with a non-form element/component
    pub fn bind<B: Serialize>(&self, name: &'static str) -> Bind<B, T> {
        Bind(name, self.clone(), PhantomData)
    }

    /// Register a form element
    pub fn register<E>(&self) -> Register<T, E> {
        Register {
            form: self.clone(),
            element_type: PhantomData,
        }
    }

    /// Get the reference for the form
    pub fn node_ref(&self) -> NodeRef {
        self.node_ref.clone()
    }
}
