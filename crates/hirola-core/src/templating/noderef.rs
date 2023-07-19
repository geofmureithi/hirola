//! References to nodes in templates.
use crate::generic_node::DomType;
use std::any::Any;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// A reference to a [`GenericNode`].
#[derive(Clone, PartialEq, Eq)]
pub struct NodeRef(Rc<RefCell<Option<DomType>>>);

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
    pub fn get(&self) -> DomType {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the T stored inside the [`NodeRef`] or `None` if it is not yet set or
    /// the wrong type.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    pub fn try_get(&self) -> Option<DomType> {
        let obj = self.0.borrow();
        (obj.as_ref()? as &dyn Any).downcast_ref().cloned()
    }

    /// Gets the raw [`DomType`] stored inside the [`NodeRef`].
    ///
    /// # Panics
    /// Panics if the [`NodeRef`] is not set yet.
    ///
    /// For a non panicking version, see [`NodeRef::try_get_raw`].
    pub fn get_raw(&self) -> DomType {
        self.try_get().expect("NodeRef is not set")
    }

    /// Tries to get the raw [`DomType`] stored inside the [`NodeRef`] or `None` if it is
    /// not yet set.
    ///
    /// For a panicking version, see [`NodeRef::get`].
    pub fn try_get_raw(&self) -> Option<DomType> {
        self.0.borrow().clone()
    }

    /// Sets the [`NodeRef`] with the specified [`DomType`].
    pub fn set(&self, node: DomType) {
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
