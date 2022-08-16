use crate::prelude::*;

use anymap::{CloneAny, Map};

type ExtensionMap = Map<dyn CloneAny>;

/// Represents an instance of a mountable app
#[derive(Clone)]
pub struct HirolaApp {
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
        let extensions = ExtensionMap::new();
        HirolaApp { extensions }
    }

    /// Fetch global data
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
    pub fn extend<T: 'static + Clone>(&mut self, extension: T) {
        self.extensions.insert(extension);
    }

    pub fn render_to_string(&self, dom: impl FnOnce(&HirolaApp) -> Dom) -> String {
        let mut ret = None;
        let _owner = create_root(|| ret = Some(format!("{:?}", dom(&self).inner_element())));

        ret.unwrap()
    }
}
