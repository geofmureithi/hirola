/// Trait for defining side effects.
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
pub trait SideEffect<Attr: EffectAttribute<Handler = Self>, Effect, Node> {
    fn effect(&self, node: &Node, attr: Attr, effect: Effect);
}

pub trait EffectAttribute {
    type Handler;
    fn read_as_attr(&self) -> String;
}
