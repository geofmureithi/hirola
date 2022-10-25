use crate::prelude::*;

#[cfg(feature = "global-state")]
#[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
use anymap::{CloneAny, Map};

#[cfg(feature = "global-state")]
#[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
type ExtensionMap = Map<dyn CloneAny>;

/// Represents an instance of a mountable app. Can be extended to hold a `Global State` and [`Router`].
#[derive(Clone)]
pub struct HirolaApp {
    #[cfg(feature = "global-state")]
    #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
    extensions: ExtensionMap,
}

/// Represents a view that can be mounted
pub trait Mountable {
    fn mount(&self, app: &HirolaApp) -> Dom;
}

pub type Dom = TemplateResult<DomType>;

#[cfg(feature = "ssr")]
pub type DomType = SsrNode;

#[cfg(not(feature = "ssr"))]
pub type DomType = DomNode;

impl<F> Mountable for F
where
    F: Fn(&HirolaApp) -> Dom,
{
    fn mount(&self, app: &HirolaApp) -> Dom {
        self(app)
    }
}

impl HirolaApp {
    /// Create a new app
    pub fn new() -> Self {
        #[cfg(feature = "global-state")]
        #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
        let extensions = ExtensionMap::new();
        HirolaApp {
            #[cfg(feature = "global-state")]
            #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
            extensions,
        }
    }

    /// Fetch global data
    #[cfg(feature = "global-state")]
    #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
    pub fn data<T>(&self) -> Option<&T>
    where
        T: Clone + 'static,
    {
        self.extensions.get::<T>()
    }

    /// Render a view
    #[cfg(not(feature = "ssr"))]
    pub fn mount<M: Mountable>(self, element: &web_sys::Node, view: M) {
        render_to(|| view.mount(&self), element);
    }

    /// Extend global data
    #[cfg(feature = "global-state")]
    #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
    pub fn extend<T: 'static + Clone>(&mut self, extension: T) {
        self.extensions.insert(extension);
    }

    #[cfg(feature = "ssr")]
    pub fn render_to_string(
        &self,
        dom: impl FnOnce(&HirolaApp) -> TemplateResult<crate::generic_node::SsrNode>,
    ) -> String {
        render_to_string(|| dom(self))
    }
}
