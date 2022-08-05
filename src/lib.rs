//! # Hirola API Documentation
//! **Hirola** is an opinionated web framework for that is focused on simplicity and predictability.
//! ## Example
//! ```rust,no_run
//! use hirola::prelude::*;
//!
//! fn counter(_: &HirolaApp) -> Dom {
//!    let state = Signal::new(99);
//!     let decerement = state.reduce_callback(|count, _| *count - 1);
//!     let incerement = state.reduce_callback(|count, _| *count + 1);
//!
//!     html! {
//!         <div class="flex flex-row h-10">
//!             <button on:click={decerement}>"-"</button>
//!             <input value={state.get()} disabled/>
//!             <button on:click={incerement}>"+"</button>
//!         </div>
//!     }
//! }
//!
//! fn main() {
//!     let app = HirolaApp::new();
//!     app.mount("body", counter);
//! }
//! ```
//!
//!
//! ## Features
//! - `dom` (_default_) - Enables rendering templates to DOM nodes. Only useful on `wasm32-unknown-unknown` target.
//! - `ssr` - Enables rendering templates to static strings (useful for Server Side Rendering / Pre-rendering).
//! - `serde` - Enables serializing and deserializing `Signal`s and other wrapper types using `serde`.
//!
//! Hirola is based on [marple reactivity core](https://github.com/lukechu10/maple).

extern crate hirola_core;

use std::{collections::HashMap, future::Future};

use anymap::{CloneAny, Map};
use hirola_core::prelude::*;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::window;

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

/// Route that is matched
#[derive(Clone, Debug)]
pub struct RouteMatch {
    //page: TemplateResult<DomNode>,
    path: String,
    pub params: HashMap<String, String>,
}

/// Represents a Single page router
#[derive(Clone)]
pub struct Router {
    current: Signal<RouteMatch>,
    inner: matchit::Router<fn(&HirolaApp) -> TemplateResult<DomNode>>,
}

impl Mountable for Router {
    fn mount(&self, app: &HirolaApp) {
        self.render(app)
    }
}

impl Router {
    pub fn new() -> Self {
        let mut path = String::from("/");
        if let Some(window) = web_sys::window() {
            path = window.location().pathname().unwrap_or("/".to_string());
        }

        Self {
            current: Signal::new(RouteMatch {
                path,
                params: HashMap::new(),
            }),
            inner: matchit::Router::new(),
        }
    }

    pub fn params(&self) -> Signal<RouteMatch> {
        self.current.clone()
    }

    /// Add a new route
    pub fn add(&mut self, path: &str, page: fn(&HirolaApp) -> Dom) {
        self.inner.insert(path.to_string(), page).unwrap();
    }

    pub fn push(&self, path: &str) {
        let window = web_sys::window().unwrap();
        window
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::default(), "", Some(&path))
            .unwrap();

        let inner = self.inner.at(&path).unwrap();
        let params = inner.params.clone();
        let params = params.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(c.0.to_string(), c.1.to_string());
            map
        });
        self.current.set(RouteMatch {
            path: path.to_owned(),
            params,
        });
    }

    fn get_fragment() -> String {
        return web_sys::window().unwrap().location().pathname().unwrap();
    }

    pub fn render(&self, app: &HirolaApp) {
        let path = (&self.current.get().path).clone();
        // let params = value.params;
        let inner = self.inner.at(&path).unwrap();
        let params = inner.params.clone();
        let params = params.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(c.0.to_string(), c.1.to_string());
            map
        });
        self.current.set(RouteMatch { path, params });

        let current = self.current.clone();

        //Hash routing forward in history and URL rewrite
        let handle_hash = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let l: String = web_sys::window()
                .unwrap()
                .location()
                .hash()
                .unwrap()
                .chars()
                .skip(1)
                .collect();

            //log(&["hash handle : ", l.as_str()].concat());

            let h = web_sys::window().unwrap().history().unwrap();
            h.replace_state_with_url(&JsValue::NULL, "", Some(l.as_str()))
                .unwrap();

            current.set(RouteMatch {
                path: l.to_string(),
                params: Default::default(),
            });
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onhashchange(Some(handle_hash.as_ref().unchecked_ref()));
        handle_hash.forget();

        let current = self.current.clone();
        //Routing for navigating in history and escaping hash routes
        let handle_pop = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let l = Self::get_fragment();

            if web_sys::window()
                .unwrap()
                .location()
                .hash()
                .unwrap()
                .chars()
                .count()
                > 0
            {
                //log("hash detected");
                return ();
            }
            current.set(RouteMatch {
                path: l.to_string(),
                params: Default::default(),
            });

            //log(&["pop handle : ", l.as_str()].concat());
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(handle_pop.as_ref().unchecked_ref()));

        handle_pop.forget();
        let route = self.current.clone();

        let mut app = app.clone();
        app.extend(self.clone()); // Add Router to data
        let router = self.inner.clone();

        create_effect(cloned!((route) => move || {
            render(|| {
                let document = window().unwrap().document().unwrap();
                let element = &document.body().unwrap();

                while let Some(child) =  element.first_child()  {
                    element.remove_child(&child).unwrap();
                }
                let path = &route.get().path;
                let value = router.at(&path).unwrap();
                let pagefn = value.value;

                pagefn(&app)
            });

        }));
    }
}

/// Helper for making async calls
pub fn use_async<F, T: 'static>(future: F) -> Signal<Option<T>>
where
    F: Future<Output = T> + 'static,
{
    let handler = Signal::new(None);
    let inner = handler.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let res = future.await;
        inner.set(Some(res));
    });
    handler
}

pub type Dom = TemplateResult<DomNode>;

pub mod prelude {
    pub use super::*;
    pub use hirola_core::prelude::*;
    pub use web_sys::Event;
}
