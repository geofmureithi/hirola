extern crate hirola_core;

use std::{collections::HashMap, ops::Deref};

use anymap::{any::CloneAny, Map};
use hirola_core::prelude::*;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

type ExtensionMap = Map<dyn CloneAny>;

pub struct HirolaApp {
    extensions: ExtensionMap,
}

pub trait Route<T> {
    fn render(self, app: &HirolaApp) -> TemplateResult<DomNode>;
}

impl<F> Route<()> for F
where
    F: FnOnce() -> TemplateResult<DomNode>,
{
    fn render(self, _app: &HirolaApp) -> TemplateResult<DomNode> {
        self()
    }
}

impl<T, K1: 'static> Route<(K1,)> for T
where
    T: FnOnce(K1) -> TemplateResult<DomNode>,
    K1: Clone + FromRequest,
{
    fn render(self, app: &HirolaApp) -> TemplateResult<DomNode> {
        let k1: K1 = FromRequest::from_request(&app);

        self(k1)
    }
}

impl<T, K1: 'static, K2: 'static> Route<(K1, K2)> for T
where
    T: FnOnce(K1, K2) -> TemplateResult<DomNode>,
    K1: FromRequest + Clone,
    K2: FromRequest + Clone,
{
    fn render(self, app: &HirolaApp) -> TemplateResult<DomNode> {
        let k1: K1 = FromRequest::from_request(&app);
        let k2: K2 = FromRequest::from_request(&app);
        self(k1, k2)
    }
}

pub trait FromRequest {
    fn from_request(app: &HirolaApp) -> Self;
}

impl<T: 'static + Clone> FromRequest for Extension<T> {
    fn from_request(app: &HirolaApp) -> Self {
        let ext = app.extensions.get::<Extension<T>>().unwrap();
        ext.clone()
    }
}

impl HirolaApp {
    pub fn new() -> Self {
        let extensions = ExtensionMap::new();
        HirolaApp { extensions }
    }

    pub fn mount(self, _element: &str, page: impl FnOnce() -> TemplateResult<DomNode>) {
        render(page)
    }

    pub fn extend<T: 'static + Clone>(&mut self, extension: T) {
        self.extensions.insert(Extension(extension));
    }

    pub fn render_to_string(&self, dom: impl FnOnce() -> TemplateResult<DomNode>) -> String {
        let mut ret = None;
        let _owner = create_root(|| ret = Some(format!("{:?}", dom().inner_element())));

        ret.unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct RouteMatch {
    //page: TemplateResult<DomNode>,
    path: String,
    pub params: HashMap<String, String>,
}

#[derive(Clone)]
pub struct Router {
    current: Signal<RouteMatch>,
    inner: matchit::Router<fn(RouteMatch) -> TemplateResult<DomNode>>,
}

impl Router {
    pub fn new() -> Self {
        let window = web_sys::window().unwrap();
        let path = window.location().pathname().unwrap();

        Self {
            current: Signal::new(RouteMatch {
                path,
                params: HashMap::new(),
            }),
            inner: matchit::Router::new(),
        }
    }

    pub fn add(&mut self, path: &str, page: fn(RouteMatch) -> TemplateResult<DomNode>) {
        self.inner.insert(path.to_string(), page).unwrap();
    }

    pub fn push(&self, path: String) {
        //let inner = self.inner.at(&path).unwrap();
        //let params = res.params.clone();
        let window = web_sys::window().unwrap();
        window
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::default(), "", Some(&path))
            .unwrap();

        // params.iter().fold(HashMap::new(), |mut map, c| {
        //     map.insert(c.0.to_string(), c.1.to_string());
        //     map
        // }
        // self.current.set(RouteMatch {
        //     path: path.to_owned(),
        //     params: HashMap::new(),
        // });
    }

    fn get_fragment() -> String {
        return web_sys::window().unwrap().location().pathname().unwrap();
    }

    pub fn render(&self) -> TemplateResult<DomNode> {
        let route = self.current.get();
        let path = &route.path;
        let value = self.inner.at(&path).unwrap();
        let pagefn = value.value;
        let path = (&route.path).clone();
        let params = value.params;

        let params = params.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(c.0.to_string(), c.1.to_string());
            map
        });

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

        pagefn(RouteMatch { path, params })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Extension<T>(pub T);

impl<T> Deref for Extension<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod prelude {
    pub use super::*;
    pub use hirola_core::prelude::*;
    pub use web_sys::Event;
}
