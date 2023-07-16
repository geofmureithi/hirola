pub mod router;
use crate::builder::Dom;
use router::Router;

#[derive(Debug, Clone)]
pub struct App<S> {
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

    pub fn middleware<NT>(self, f: impl Fn(Self) -> NT) -> App<NT> {
        App {
            state: f(self),
            router: self.router,
        }
    }
}

#[cfg(feature = "dom")]
impl<S: Clone + 'static> App<S> {
    pub fn mount(&mut self, parent: &web_sys::Node) {
        let router = self.router.clone();
        let dom = router.render(
            self,
            &crate::generic_node::DomNode {
                node: parent.clone(),
            },
        );
        std::mem::forget(dom);
    }
}

#[cfg(feature = "ssr")]
impl<S: Clone + 'static> App<S> {
    pub fn render_to_string(&self, path: &str) -> String {
        use crate::generic_node::GenericNode;
        let fragment = crate::generic_node::SsrNode::fragment();
        let router = self.handler.clone();
        // Set the path
        router.handler.push(path);
        // Render path to fragment
        router.render(self, &fragment);
        format!("{fragment}")
    }
}
