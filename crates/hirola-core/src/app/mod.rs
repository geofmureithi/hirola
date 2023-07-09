use std::ops::{Deref, DerefMut};

#[cfg(feature = "router")]
use crate::router::{Router, RouterHandle};

#[derive(Debug, Clone)]
pub struct App<S> {
    #[cfg(feature = "router")]
    router: Router<Self>,
    state: S,
}

impl<S: Clone + 'static> App<S> {
    pub fn new(state: S) -> Self {
        Self {
            #[cfg(feature = "router")]
            router: Router::new(),
            state,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    #[cfg(feature = "router")]
    pub fn mount(&mut self, parent: &web_sys::Node) {
        let router = self.router.clone();
        router.render(self, parent);
    }

    #[cfg(feature = "router")]
    pub fn router(&self) ->  &RouterHandle {
        &self.router.handler
    }
}

#[cfg(feature = "router")]
impl<S> Deref for App<S> {
    type Target = Router<Self>;
    fn deref(&self) -> &Self::Target {
        &self.router
    }
}

#[cfg(feature = "router")]
impl<S> DerefMut for App<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.router
    }
}
