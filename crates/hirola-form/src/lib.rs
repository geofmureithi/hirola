use std::collections::BTreeMap;

use hirola_core::{
    effect::EffectAttribute,
    generic_node::EventListener,
    prelude::{GenericNode, MutableBTreeMap},
};
use hirola_dom::effects::attr_mixin::XEffect;
use hirola_dom::{node_ref::NodeRef, Dom};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement};

pub struct Form;

impl EffectAttribute for Form {
    type Handler = XEffect;
    fn read_as_attr(&self) -> String {
        "form".to_string()
    }
}

/// Form plugin for hirola
#[derive(Clone, Debug)]
pub struct FormHandler<T: FormEntity> {
    node_ref: NodeRef,
    pub value: MutableBTreeMap<T::Columns, String>,
}

impl<T: FormEntity + Clone> FormHandler<T> {
    /// Build a new reactive form
    pub fn new(value: T) -> Self {
        Self {
            node_ref: NodeRef::new(),
            value: MutableBTreeMap::with_values(value.into()),
        }
    }

    pub fn update_field(&self, name: T::Columns, value: String) {
        self.value.lock_mut().insert_cloned(name, value);
    }

    /// Get and cast a field value
    pub fn get_value_by_field(&self, name: &T::Columns) -> Option<String> {
        self.value.lock_ref().get(name).cloned()
    }
}
impl<T: FormEntity + Clone> FormHandler<T> {
    pub fn current(&self) -> T {
        self.value.lock_ref().clone().into()
    }
}

pub trait FormColumn {
    fn name(&self) -> &str;
}

pub trait FormEntity:
    From<BTreeMap<Self::Columns, String>> + Into<BTreeMap<Self::Columns, String>>
{
    type Columns: FormColumn + Copy + Ord;
}

impl<T: FormEntity + Clone> FormHandler<T>
where
    T::Columns: 'static,
    T: 'static,
{
    pub fn bind(&self, column: T::Columns) -> Box<dyn FnOnce(&Dom)> {
        let form = self.clone();
        let cb = move |dom: &Dom| {
            let f = form.clone();
            let handler = Box::new(move |e: Event| {
                let input = e
                    .current_target()
                    .unwrap()
                    .dyn_into::<HtmlInputElement>()
                    .unwrap();
                let new_value = input.value();
                f.update_field(column, new_value);
            });
            dom.event("input", handler);
            let value: String = form.get_value_by_field(&column).unwrap_or_default();
            dom.set_attribute("value", &value);

            dom.set_attribute("name", &column.name());
        };

        Box::new(cb)
    }

    /// Get the reference for the form
    pub fn node_ref(&self) -> NodeRef {
        self.node_ref.clone()
    }
}
