use std::sync::Arc;
use crate::Dom;

pub trait State {}

pub trait Props {}

pub trait Component<P, S>
where
    S: State,
    P: Props,
{
    fn render(&mut self, state: &Arc<S>) -> Dom;
    fn before_mount(&mut self, _props: &P, _state: &Arc<S>) {
        // before the component is mounted
    }
    fn on_mount(&mut self, _props: &P, _state: &Arc<S>) {
        // right after the component is mounted on the page
    }
}
