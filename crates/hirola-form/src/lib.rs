use std::{marker::PhantomData, rc::Rc};

use hirola_core::{
    update::MixinError,
    cloned,
    prelude::{create_effect, Mutable, ReadOnlyMutable},
    prelude::{DomNode, DomType, GenericNode, Mixin, NodeRef, State},
};
use json_dotpath::DotPaths;
use serde::{de::DeserializeOwned, Serialize};
use validator::{Validate, ValidationErrors};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlSelectElement};

/// Form plugin for hirola
#[derive(Clone, Debug)]
pub struct FormHandler<T: 'static> {
    node_ref: NodeRef<DomType>,
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
    pub fn update_field<S: Serialize>(&self, name: &str, value: S) {
        let current_value = self.value.clone();
        let mut json = serde_json::to_value(&current_value).unwrap();
        json.dot_set(&name, value).unwrap();
        let ser: T = serde_json::from_value(json).unwrap();
        current_value.set(ser);
    }

    /// Get and cast a field value
    pub fn get_value_by_field<S: DeserializeOwned>(
        &self,
        name: &str,
    ) -> Result<Option<S>, json_dotpath::Error> {
        let current_value = self.value.clone();
        let json = serde_json::to_value(&current_value)?;
        json.dot_get(name)
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

impl<T: Serialize + DeserializeOwned + Clone> Mixin for Register<T, HtmlInputElement> {
    fn mixin(&self, ns: &str, node: DomNode) -> Result<(), MixinError> {
        if ns != "form" {
            return Err(MixinError::InvalidNamespace {
                expected: "form".to_string(),
                found: ns.to_string(),
            });
        }

        let form = self.form.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();
            let name = input.name();

            let new_value = input.value();
            form.update_field(&name, new_value);
        });
        node.event("input", handler);
        let input = {
            let node = node.clone();
            node.dyn_into::<HtmlInputElement>()
                .map_err(MixinError::NodeError)?
        };
        let name = input.name();
        let value: String = self.form.get_value_by_field(&name).unwrap().unwrap();
        node.set_attribute("value", &value);
        Ok(())
    }
}

impl<T: Serialize + DeserializeOwned + Clone> Mixin for Register<T, HtmlSelectElement> {
    fn mixin(&self, ns: &str, node: DomNode) -> Result<(), MixinError> {
        if ns != "form" {
            // compile_error!("This will never work");
            return Err(MixinError::InvalidNamespace {
                expected: "form".to_string(),
                found: ns.to_string(),
            });
        }

        let form = self.form.clone();
        let handler = Box::new(move |e: Event| {
            let input = e
                .current_target()
                .unwrap()
                .dyn_into::<HtmlSelectElement>()
                .unwrap();
            let name = input.name();

            let new_value = input.value();
            form.update_field(&name, new_value);
        });
        node.event("change", handler);
        Ok(())
    }
}

/// Allows connecting non-standard elements(eg components) to forms
#[derive(Clone, Debug)]
pub struct Bind<B: 'static, F: 'static>(&'static str, FormHandler<F>, PhantomData<B>);

impl<B: Serialize + DeserializeOwned, F: Serialize + DeserializeOwned + Clone> Bind<B, F> {
    /// Manually set the value of the bound field
    pub fn set_value(&self, value: B) {
        self.1.update_field(self.0, value);
    }

    /// Get value of bound field
    pub fn get_value(&self) -> Mutable<B> {
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
        let signal = Mutable::new(read_inner_value::<F, B>(&current_value.get_cloned(), name));
        let signal_ret = signal.clone();
        create_effect(current_value, move |value| {
            signal.set(read_inner_value(&value, name))
        });
        signal_ret
    }
}

impl<B: Clone, F: Clone> State for Bind<B, F> {}

impl<T: Validate + Clone> FormHandler<T> {
    /// Perform validation
    pub fn validate(&self) -> Result<(), ValidationErrors> {
        self.value.get_cloned().validate()
    }

    /// Get error specific field
    pub fn error_for(&self, name: &'static str) -> Mutable<String> {
        let signal = Mutable::new(String::new());
        let value = self.value.clone();
        let ret_signal = signal.clone();
        create_effect(value.clone(), move |v| {
            let res = value.get_cloned().validate();
            if ValidationErrors::has_error(&res, name) {
                let err = res.err().unwrap();
                let err = err.field_errors();
                let value = err.get(name).unwrap().first();
                if let Some(v) = value {
                    signal.set(format!("{}", v.message.clone().unwrap()))
                } else {
                    signal.set(String::new())
                }
            } else {
                signal.set(String::new())
            }
        });
        ret_signal
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
    pub fn node_ref(&self) -> NodeRef<DomType> {
        self.node_ref.clone()
    }
}
