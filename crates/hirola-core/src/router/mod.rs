use std::{
    cell::RefCell, collections::HashMap, future::IntoFuture, marker::PhantomData, rc::Rc, sync::Arc,
};

use crate::{prelude::*, builder::{ViewBuilder, fragment::Fragment}, view::View};
use discard::DiscardOnDrop;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{window, Element, Event, HtmlLinkElement, Node};

#[derive(Clone, Debug)]
pub struct RouterHandle {
    current: Mutable<String>,
}
impl RouterHandle {
    pub fn push(&self, path: &str) {
        let window = web_sys::window().unwrap();
        window
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::default(), "", Some(&path))
            .unwrap();
        self.current.set(path.to_owned());
    }

    pub fn link(&self) -> Box<dyn Fn(&View<DomNode>) -> () + '_> {
        let router: RouterHandle = self.clone();
        let cb = move |node: &View<DomNode>| {
            let router = router.clone();
            let handle_click = Box::new(move |e: Event| {
                e.prevent_default();

                let element = e.current_target().unwrap().dyn_into::<Element>().unwrap();

                let href = element.get_attribute("href").unwrap();

                router.push(&href);
            }) as Box<dyn Fn(Event)>;

            node.event("click", handle_click);
        };
        Box::new(cb)
    }
}

/// Represents a Single page router
#[derive(Clone)]
pub struct Router<A> {
    pub (crate) handler: RouterHandle,
    inner: matchit::Router<fn(&A) -> ViewBuilder<DomNode>>,
    app: PhantomData<A>,
}

impl<A> std::fmt::Debug for Router<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Router")
            .field("current", &self.handler)
            .finish()
    }
}

impl<A: 'static + Clone> Router<A> {
    pub fn new() -> Self {
        let mut path = String::from("/");
        if let Some(window) = web_sys::window() {
            path = window.location().pathname().unwrap_or("/".to_string());
        }

        Self {
            handler: RouterHandle {
                current: Mutable::new(path),
            },
            inner: matchit::Router::new(),
            app: PhantomData,
        }
    }

    pub fn current_params(&self) -> HashMap<String, String> {
        let path = self.handler.current.get_cloned();
        let binding = &self.inner;
        let inner = binding.at(&path).unwrap();
        let params = &inner.params.clone();
        let params = params.iter().fold(HashMap::new(), |mut map, c| {
            map.insert(c.0.to_string(), c.1.to_string());
            map
        });
        params
    }


    /// Add a new route
    pub fn route(&mut self, path: &str, page: fn(&A) -> ViewBuilder<DomNode>) {
        self.inner.insert(path.to_string(), page).unwrap();
    }


    fn get_fragment() -> String {
        return web_sys::window().unwrap().location().pathname().unwrap();
    }

    pub fn link(&self) -> Box<dyn Fn(&View<DomNode>) -> () + '_> {
        let router: RouterHandle = self.handler.clone();
        let cb = move |node: &View<DomNode>| {
            let router = router.clone();
            let handle_click = Box::new(move |e: Event| {
                e.prevent_default();

                let element = e.current_target().unwrap().dyn_into::<Element>().unwrap();

                let href = element.get_attribute("href").unwrap();

                router.push(&href);
            }) as Box<dyn Fn(Event)>;

            node.event("click", handle_click);
        };
        Box::new(cb)
    }

    pub (crate) fn render(&self, app: &A, parent: &Node) {
        let router = &self.inner;
        let current = self.handler.current.clone();

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

            log::debug!("hash handle : {l}");

            let h = web_sys::window().unwrap().history().unwrap();
            h.replace_state_with_url(&JsValue::NULL, "", Some(l.as_str()))
                .unwrap();

            current.set(l.to_string());
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onhashchange(Some(handle_hash.as_ref().unchecked_ref()));
        handle_hash.forget();

        let current = self.handler.current.clone();
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
                log::debug!("hash detected");
                return ();
            }
            current.set(l.to_string());
            log::debug!("pop handle : {l}");
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(handle_pop.as_ref().unchecked_ref()));

        handle_pop.forget();
        let route = (&self.handler).current.clone();
        let router = router.clone();
        let node = parent.clone().dyn_into::<web_sys::HtmlElement>().unwrap();
        let app = app.clone();
        let next = route
        .signal_cloned().map(move|route_match| {
            let match_result = router.at(&route_match).unwrap();
            let page_fn = match_result.value;

            let builder = page_fn(&app);
            let view = builder.mount(&DomNode::fragment()).unwrap();
            node.replace_children_with_node_1(&view.node().inner_element());
            let window = web_sys::window().unwrap();
                window
                    .history()
                    .unwrap()
                    .push_state_with_url(&JsValue::default(), "", Some(&route_match))
                    .unwrap();
                log::debug!("Router received new path: {route_match}");
        }).to_future();
        wasm_bindgen_futures::spawn_local(next);
    }
}
