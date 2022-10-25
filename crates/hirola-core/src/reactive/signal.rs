use crate::callback::{State, StateReduce};

use super::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

/// A readonly [`Signal`].
///
/// Returned by functions that provide a handle to access state.
/// Use [`Signal::handle`] or [`Signal::into_handle`] to retrieve a handle from a [`Signal`].

pub struct StateHandle<T: 'static>(Rc<RefCell<SignalInner<T>>>);

impl<T: 'static> StateHandle<T> {
    /// Get the current value of the state.
    pub fn get(&self) -> Rc<T> {
        // if inside an effect, add this signal to dependency list
        CONTEXTS.with(|contexts| {
            if let Some(last_context) = contexts.borrow().last() {
                let signal = Rc::downgrade(&self.0);

                last_context
                    .upgrade()
                    .expect("Running should be valid while inside reactive scope")
                    .borrow_mut()
                    .as_mut()
                    .unwrap()
                    .dependencies
                    .insert(Dependency(signal));
            }
        });

        self.get_untracked()
    }

    /// Get the current value of the state, without tracking this as a dependency if inside a
    /// reactive context.
    ///
    /// # Example
    ///
    /// ```
    /// use hirola_core::prelude::*;
    ///
    /// let state = Signal::new(1);
    ///
    /// let double = create_memo({
    ///     let state = state.clone();
    ///     move || *state.get_untracked() * 2
    /// });
    ///
    /// assert_eq!(*double.get(), 2);
    ///
    /// state.set(2);
    /// // double value should still be old value because state was untracked
    /// assert_eq!(*double.get(), 2);
    /// ```
    pub fn get_untracked(&self) -> Rc<T> {
        Rc::clone(&self.0.borrow().inner)
    }
}

impl<T: 'static> Clone for StateHandle<T> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T: fmt::Debug> fmt::Debug for StateHandle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("StateHandle")
            .field(&self.get_untracked())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for StateHandle<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get_untracked().as_ref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for StateHandle<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Signal::new(T::deserialize(deserializer)?).handle())
    }
}

/// State that can be set.
///
/// # Example
/// ```
/// use hirola_core::prelude::*;
///
/// let state = Signal::new(0);
/// assert_eq!(*state.get(), 0);
///
/// state.set(1);
/// assert_eq!(*state.get(), 1);
/// ```
pub struct Signal<T: 'static> {
    handle: StateHandle<T>,
}

impl<T: 'static + Clone> Signal<Vec<T>> {
    pub fn replace(&self, values: Vec<T>) {
        self.set(values);
    }

    pub fn insert(&self, index: usize, value: T) {
        let mut inner = (&*self.get_untracked()).clone();
        inner.insert(index, value);
        self.set(inner);
    }

    // pub fn update(&self, index: usize, value: T) {
    //     // let mut inner = *self.get_untracked();
    //     // inne;
    //     // self.set(inner);
    // }

    pub fn remove(&self, index: usize) {
        let mut inner = (&*self.get_untracked()).clone();
        inner.remove(index);
        self.set(inner);
    }

    pub fn swap(&self, index1: usize, index2: usize) {
        let mut inner = (&*self.get_untracked()).clone();
        inner.swap(index1, index2);
        self.set(inner);
    }

    pub fn push(&self, value: T) {
        let mut inner = (&*self.get()).clone();
        inner.push(value);
        self.set(inner);
    }

    pub fn pop(&self) {
        let mut inner = (&*self.get_untracked()).clone();
        inner.pop();
        self.set(inner);
    }

    pub fn clear(&self) {
        let mut inner = (&*self.get_untracked()).clone();
        inner.clear();
        self.set(inner);
    }
}

impl<T: 'static> Signal<T> {
    /// Creates a new signal with the given value.
    ///
    /// # Example
    /// ```
    /// # use hirola_core::prelude::*;
    /// let state = Signal::new(0);
    /// # assert_eq!(*state.get(), 0);
    /// ```
    pub fn new(initial: T) -> Self {
        Self {
            handle: StateHandle(Rc::new(RefCell::new(SignalInner::new(initial)))),
        }
    }

    /// Set the current value of the state.
    ///
    /// This will notify and update any [`create_effect`] and [`create_memo`] that depend on this value.
    ///
    /// # Example
    /// ```
    /// # use hirola_core::prelude::*;
    /// 
    /// let state = Signal::new(0);
    /// assert_eq!(*state.get(), 0);
    ///
    /// state.set(1);
    /// assert_eq!(*state.get(), 1);
    /// ```
    pub fn set(&self, new_value: T) {
        self.handle.0.borrow_mut().update(new_value);

        self.trigger_subscribers();
    }

    /// Get the [`StateHandle`] associated with this signal.
    ///
    /// This is a shortcut for `(*signal).clone()`.
    pub fn handle(&self) -> StateHandle<T> {
        self.handle.clone()
    }

    /// Consumes this signal and returns its underlying [`StateHandle`].
    pub fn into_handle(self) -> StateHandle<T> {
        self.handle
    }

    /// Calls all the subscribers without modifying the state.
    /// This can be useful when using patterns such as inner mutability where the state updated will not be automatically triggered.
    /// In the general case, however, it is preferable to use [`Signal::set`] instead.
    pub fn trigger_subscribers(&self) {
        // Clone subscribers to prevent modifying list when calling callbacks.
        let subscribers = self.handle.0.borrow().subscribers.clone();

        for subscriber in subscribers {
            // subscriber might have already been destroyed in the case of nested effects
            if let Some(callback) = subscriber.try_callback() {
                callback()
            }
        }
    }
}

impl<T> StateReduce<T> for Signal<T> {
    /// Sets the return value
    /// 
    /// # Example
    /// ```
    /// use hirola::prelude::*;
    /// use web_sys::Event;
    /// 
    /// pub struct PageState{
    ///     pub counter: Signal<u32>
    /// }
    /// 
    /// impl PageState {
    ///     fn add_one(&self) -> Box<dyn Fn(Event)> {
    ///         self.counter.mut_callback(move |counter, _e: Event| {
    ///             *counter + 1
    ///         })
    ///     }
    /// }
    /// 
    /// pub fn your_page(_app: HirolaApp) -> Dom {
    ///     let state = PageState{counter: Signal::new(0)};
    ///     let add_one = state.add_one();
    ///     
    ///     html! {
    ///         <div>
    ///             <p>"counter is: "{state.counter.get()}</p>
    ///             <button on:click=add_one>"add one"</button>
    ///         </div>
    ///     }
    /// }
    /// ```
    fn mut_callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(&T, E) -> T + 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            state.set(f(&state.get(), e));
        };
        Box::new(cb)
    }
}

impl<T> State for Signal<T> {}

impl<T: 'static> Deref for Signal<T> {
    type Target = StateHandle<T>;

    fn deref(&self) -> &Self::Target {
        &self.handle
    }
}

impl<T: 'static> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl<T: PartialEq> PartialEq for Signal<T> {
    fn eq(&self, other: &Signal<T>) -> bool {
        self.get_untracked().eq(&other.get_untracked())
    }
}

impl<T: fmt::Debug> fmt::Debug for Signal<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Signal")
            .field(&self.get_untracked())
            .finish()
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Signal<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get_untracked().as_ref().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Signal<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Signal::new(T::deserialize(deserializer)?))
    }
}

pub(super) struct SignalInner<T> {
    inner: Rc<T>,
    subscribers: HashSet<Callback>,
}

impl<T> SignalInner<T> {
    fn new(value: T) -> Self {
        Self {
            inner: Rc::new(value),
            subscribers: HashSet::new(),
        }
    }

    /// Adds a handler to the subscriber list. If the handler is already a subscriber, does nothing.
    fn subscribe(&mut self, handler: Callback) {
        self.subscribers.insert(handler);
    }

    /// Removes a handler from the subscriber list. If the handler is not a subscriber, does nothing.
    fn unsubscribe(&mut self, handler: &Callback) {
        self.subscribers.remove(handler);
    }

    /// Updates the inner value. This does **NOT** call the subscribers.
    /// You will have to do so manually with `trigger_subscribers`.
    fn update(&mut self, new_value: T) {
        self.inner = Rc::new(new_value);
    }
}

/// Trait for any [`SignalInner`], regardless of type param `T`.
pub(super) trait AnySignalInner {
    /// Wrapper around [`SignalInner::subscribe`].
    fn subscribe(&self, handler: Callback);
    /// Wrapper around [`SignalInner::unsubscribe`].
    fn unsubscribe(&self, handler: &Callback);
}

impl<T> AnySignalInner for RefCell<SignalInner<T>> {
    fn subscribe(&self, handler: Callback) {
        self.borrow_mut().subscribe(handler);
    }

    fn unsubscribe(&self, handler: &Callback) {
        self.borrow_mut().unsubscribe(handler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signals() {
        let state = Signal::new(0);
        assert_eq!(*state.get(), 0);

        state.set(1);
        assert_eq!(*state.get(), 1);
    }

    #[test]
    fn signal_composition() {
        let state = Signal::new(0);

        let double = || *state.get() * 2;

        assert_eq!(double(), 0);

        state.set(1);
        assert_eq!(double(), 2);
    }

    #[test]
    fn state_handle() {
        let state = Signal::new(0);
        let readonly = state.handle();

        assert_eq!(*readonly.get(), 0);

        state.set(1);
        assert_eq!(*readonly.get(), 1);
    }
}
