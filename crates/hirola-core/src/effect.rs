use std::future::Future;

use crate::BoxedLocal;

/// Trait for defining side effects that execute asynchronously as futures.
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
pub trait SideEffect {
    /// Executes the side effect and returns a boxed future representing its completion.
    ///
    /// This method executes the side effect asynchronously and returns a boxed future that
    /// represents the completion of the task. Implementations should ensure that the future's
    /// output is `()`, indicating the task's successful completion.
    ///
    /// # Returns
    ///
    /// A boxed future that represents the completion of the side effect task.
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
    fn effect(self) -> BoxedLocal<()>;
}

impl<F: 'static> SideEffect for F
where
    F: Future<Output = ()>,
{
    /// Converts the provided future into a boxed future of `()` as a side effect.
    ///
    /// This implementation allows any future that produces `()` as its output to be converted
    /// into a `BoxedLocal<()>` to fulfill the requirements of the `SideEffect` trait.
    ///
    /// # Returns
    ///
    /// A boxed future that represents the completion of the provided future task.
    ///
    /// # Example
    ///
    /// ```
    /// use std::future::ready;
    /// use hirola::prelude::*;
    /// // Create a future that produces `()` as its output
    /// let my_future = ready(());
    ///
    /// // Convert the future into a boxed side effect
    /// let boxed_side_effect = my_future.effect();
    /// 
    /// //let render = html! {
    /// //    <div use:boxed_side_effect />
    /// //};
    /// ```
    fn effect(self) -> BoxedLocal<()> {
        Box::pin(self)
    }
}

