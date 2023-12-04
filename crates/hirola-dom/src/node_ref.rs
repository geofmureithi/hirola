//! References to nodes in templates.
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use hirola_core::generic_node::NodeReference;

use crate::Dom;

/// A reference to a [`GenericNode`].
#[derive(Clone, PartialEq, Eq)]
pub struct NodeRef(Rc<RefCell<Option<Dom>>>);

impl NodeRef {
    /// Creates an empty [`NodeRef`].
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(None)))
    }

    /// Gets the T stored inside the [`NodeRef`].
    ///
    /// # Panics
    /// Panics if the [`NodeRef`] is not set yet or is the wrong type.
    ///
    /// For a non panicking version, see [`NodeRef::try_get`].
    pub fn get(&self) -> Dom {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the T stored inside the [`NodeRef`] or `None` if it is not yet set or
    /// the wrong type.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    fn inner_try_get(&self) -> Option<Dom> {
        let obj = self.0.borrow();
        (obj.as_ref()? as &dyn Any).downcast_ref().cloned()
    }

    /// Gets the raw [`DomNode`] stored inside the [`NodeRef`].
    ///
    /// # Panics
    /// Panics if the [`NodeRef`] is not set yet.
    ///
    /// For a non panicking version, see [`NodeRef::try_get_raw`].
    pub fn get_raw(&self) -> Dom {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the raw [`DomNode`] stored inside the [`NodeRef`] or `None` if it is
    /// not yet set.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    pub fn try_get_raw(&self) -> Option<Dom> {
        self.0.borrow().clone()
    }

    /// Sets the [`NodeRef`] with the specified [`DomNode`].
    fn inner_set(&self, node: Dom) {
        *self.0.borrow_mut() = Some(node);
    }
}

impl Default for NodeRef {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for NodeRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("NodeRef").field(&self.0.borrow()).finish()
    }
}

impl NodeReference for NodeRef {
    type Target = Dom;
    fn set(&self, node: Self::Target) {
        self.inner_set(node)
    }
    fn try_get(&self) -> Option<Self::Target> {
        self.inner_try_get()
    }
}
