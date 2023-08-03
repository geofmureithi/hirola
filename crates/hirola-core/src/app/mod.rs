pub mod router;
use router::Router;
use std::fmt::Debug;

use crate::dom::Dom;

#[derive(Debug, Clone)]
pub struct App<S: 'static> {
    router: Router<S>,
    state: S,
}

impl<S: Clone + 'static> App<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            router: Router::new(),
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn router(&self) -> &Router<S> {
        &self.router
    }

    /// Add a new route
    pub fn route(&mut self, path: &str, page: fn(&Self) -> Dom) {
        self.router.handler.insert(path.to_string(), page).unwrap();
    }

    pub fn set_not_found(&mut self, page: fn(&Self) -> Dom) {
        self.router.not_found = Box::new(page);
    }

    // pub fn middleware<NT>(self, f: impl Fn(Self) -> NT) -> App<NT> {
    //     App {
    //         state: f(self),
    //         router: self.router,
    //     }
    // }
}

#[cfg(feature = "dom")]
impl<S: Clone + 'static> App<S> {
    pub fn mount(&self) -> Dom {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let router = self.router.clone();
        let dom = router.render(
            &self,
            &crate::generic_node::DomNode {
                node: document.body().unwrap().into(),
            },
        );
        dom
    }

    pub fn mount_to(&self, parent: &web_sys::Node) -> Dom {
        let router = self.router.clone();
        let dom = router.render(
            &self,
            &crate::generic_node::DomNode {
                node: parent.clone(),
            },
        );
        dom
    }
}

#[cfg(feature = "ssr")]
impl<S: Clone + 'static> App<S> {
    pub fn render_to_string(&self, path: &str) -> String {
        use crate::generic_node::GenericNode;
        let fragment = crate::generic_node::SsrNode::fragment();
        let router = self.router();
        // Set the path
        router.push(path);
        // Render path to fragment
        router.render(self, &fragment);
        format!("{fragment}")
    }
}
