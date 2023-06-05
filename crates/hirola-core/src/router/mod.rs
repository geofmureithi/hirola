use std::collections::HashMap;

use crate::prelude::*;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event, HtmlLinkElement};

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
    inner: matchit::Router<fn(&HirolaApp) -> Dom>,
}

impl std::fmt::Debug for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("current", &self.current)
            .finish()
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

    pub fn param(&self, name: &str) -> Option<String> {
        let params = self.params().get();
        let params = params.params.clone();
        params.get(name).cloned()
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

    pub fn link<'a>(&'a self) -> Box<dyn Fn(DomNode) -> () + 'a> {
        let router = self.clone();
        let cb = move |node: DomNode| {
            let element = node.unchecked_into::<HtmlLinkElement>();

            let router = router.clone();
            let handle_click = Closure::wrap(Box::new(move |e: Event| {
                e.prevent_default();

                let element = e.current_target().unwrap().dyn_into::<Element>().unwrap();

                let href = element.get_attribute("href").unwrap();

                router.push(&href);
            }) as Box<dyn Fn(Event)>);


            element
                .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
                .unwrap();

            handle_click.forget();
        };
        Box::new(cb)
    }

    pub fn render(&self, app: &HirolaApp) -> Dom {
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
        let router = self.inner.clone();
        let path = &route.get().path;
        let value = router.at(&path).unwrap();
        let pagefn = value.value;
        pagefn(&app)
    }
}
