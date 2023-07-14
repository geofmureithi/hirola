use crate::router::{Router, RouterHandle};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct App<S> {
    router: Router<Self>,
    state: S,
}

impl<S: Clone + 'static> App<S> {
    pub fn new(state: S) -> Self {
        Self {
            router: Router::new(),
            state,
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn router(&self) -> &RouterHandle {
        &self.router.handler
    }

    pub fn middleware<NT>(self, f: impl Fn(Self) -> NT) -> App<NT> {
        App {
            router: self.router.coerce(),
            state: f(self),
        }
    }
}

#[cfg(feature = "dom")]
impl<S: Clone + 'static> App<S> {
    pub fn mount(&mut self, parent: &web_sys::Node) {
        let router = self.router.clone();
        let view = router.render(
            self,
            &crate::generic_node::DomNode {
                node: parent.clone(),
            },
        );
        std::mem::forget(view);
    }
}

#[cfg(feature = "ssr")]
impl<S: Clone + 'static> App<S> {
    pub fn render_to_string(&self, path: &str) -> String {
        use crate::generic_node::GenericNode;
        let fragment = crate::generic_node::SsrNode::fragment();
        let router = self.router.clone();
        // Set the path
        router.handler.push(path);
        // Render path to fragment
        router.render(self, &fragment);
        format!("{fragment}")
    }
}

impl<S> Deref for App<S> {
    type Target = Router<Self>;
    fn deref(&self) -> &Self::Target {
        &self.router
    }
}

impl<S> DerefMut for App<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.router
    }
}
