use std::future::Future;

use crate::BoxedLocal;

pub trait SideEffect {
    fn effect(self) -> BoxedLocal<()>;
}

impl<F: 'static> SideEffect for F
where
    F: Future<Output = ()>,
{
    fn effect(self) -> BoxedLocal<()> {
        Box::pin(self)
    }
}
