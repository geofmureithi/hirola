use crate::prelude::*;

#[cfg(feature = "global-state")]
#[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
use anymap::{CloneAny, Map};

#[cfg(feature = "global-state")]
#[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
type ExtensionMap = Map<dyn CloneAny>;

/// Represents an instance of a mountable app
#[derive(Clone)]
pub struct HirolaApp {
    #[cfg(feature = "global-state")]
    #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
    extensions: ExtensionMap,
}

/// Represents a view that can be mounted
pub trait Mountable {
    fn mount(&self, app: &HirolaApp);
}

pub type Dom = TemplateResult<DomNode>;

impl<F> Mountable for F
where
    F: Fn(&HirolaApp) -> Dom,
{
    fn mount(&self, app: &HirolaApp) {
        render(|| self(app));
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
    pub fn mount<M: Mountable>(self, _element: &str, view: M) {
        // let app = self.clone();
        view.mount(&self)
    }

    /// Extend global data
    #[cfg(feature = "global-state")]
    #[cfg_attr(docsrs, doc(cfg(feature = "global-state")))]
    pub fn extend<T: 'static + Clone>(&mut self, extension: T) {
        self.extensions.insert(extension);
    }

    pub fn render_to_string(&self, dom: impl FnOnce(&HirolaApp) -> Dom) -> String {
        let mut ret = None;
        let _owner = create_root(|| ret = Some(format!("{:?}", dom(&self).inner_element())));

        ret.unwrap()
    }
}
