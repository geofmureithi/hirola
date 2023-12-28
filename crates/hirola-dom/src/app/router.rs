use hirola_core::prelude::signal::{Mutable, MutableSignalCloned, SignalExt};
use hirola_core::prelude::*;
use std::collections::HashMap;
use std::fmt;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{Element, Event};

use crate::Dom;

use super::App;

/// Router struct for handling routing in the frontend application.
///
/// This struct manages the routing functionality for the frontend application. It keeps track of
/// the current route, the registered route handlers, and the not-found page handler. The `Router`
/// is parameterized over the state type `S`, which allows it to interact with the `App` state when
/// handling routes.
///
/// # Example
///
/// ```no_run
/// use hirola::prelude::*;
/// use hirola::dom::Dom;
/// use hirola_dom::app::App;
/// #[derive(Clone)]
/// struct AppState {
///     // ... fields and methods for your application state ...
/// }
///
/// fn home_page(app: &App<AppState>) -> Dom {
///     html! {<h1>"Home"</h1>}
/// }
///
/// fn about_page(app: &App<AppState>) -> Dom {
///     html! {<h1>"Home"</h1>}
/// }
///
/// let mut app = App::new(AppState { /* ... */ });
/// app.route("/", home_page);
/// app.route("/about", about_page);
/// app.mount();
/// ```
#[derive(Clone)]
pub struct Router<S: 'static = ()> {
    current: Mutable<String>,
    /// The internal router used to map route paths to corresponding route handler functions.
    pub(crate) handler: matchit::Router<fn(&App<S>) -> Dom>,
    /// The function that will be executed when the requested route does not match any registered routes.
    pub(crate) not_found: Box<fn(&App<S>) -> Dom>,
}

impl<S> fmt::Debug for Router<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("current", &self.current)
            .field(
                "handler",
                &format_args!("matchit::Router<fn(&App<S>) -> Dom>"),
            )
            .finish()
    }
}
impl<S: Clone + 'static> Router<S> {
    /// Creates a new instance of the Router with default settings.
    ///
    /// The `Router` manages the routing functionality for the frontend application. This method
    /// creates a new instance of the `Router` with an empty route handler and a default not-found
    /// page handler.
    ///
    /// # Returns
    ///
    /// A new instance of `Router<S>`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::prelude::*;
    /// use hirola::dom::app::router::Router;
    /// let router = Router::<()>::new();
    /// ```
    pub fn new() -> Self {
        let mut path = String::from("/");
        if let Some(window) = web_sys::window() {
            path = window.location().pathname().unwrap_or("/".to_string());
        }
        Router {
            current: Mutable::new(path),
            handler: Default::default(),
            not_found: Box::new(|_| Dom::text_node("Not Found")),
        }
    }

    /// Retrieves the current parameters from the current route.
    ///
    /// This method returns a HashMap containing the parameters parsed from the current route
    /// URL. The parameters are extracted from the route's path segments based on the route pattern
    /// defined during registration.
    ///
    /// # Returns
    ///
    /// A HashMap with parameter names as keys and their corresponding values as values.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// let router = Router::<()>::new();
    /// let params = router.current_params();
    /// ```
    pub fn current_params(&self) -> HashMap<String, String> {
        let path = self.current.get_cloned();
        let binding = &self.handler;
        match binding.at(&path) {
            Ok(inner) => {
                let params = &inner.params.clone();
                let params = params.iter().fold(HashMap::new(), |mut map, c| {
                    map.insert(c.0.to_string(), c.1.to_string());
                    map
                });
                params
            }
            Err(_) => HashMap::new(),
        }
    }

    /// Navigates to the specified route path.
    ///
    /// This method updates the current route to the provided `path`. It will trigger the
    /// rendering process for the new route and update the application's UI accordingly.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the route to navigate to, a string representing the route pattern.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// let router = Router::<()>::new();
    /// router.push("/about");
    pub fn push(&self, path: &str) {
        let window = web_sys::window().unwrap();
        window
            .history()
            .unwrap()
            .push_state_with_url(&JsValue::default(), "", Some(path))
            .unwrap();
        self.current.set(path.to_owned());
    }

    /// Generates a link handler function that can be used to navigate to a specific route.
    ///
    /// This method returns a boxed closure that takes a reference to a DOM element (`Dom`) and
    /// updates the current route to the specified path when triggered. It can be used to create
    /// link handlers for HTML elements, such as anchors (`<a>`), buttons, or custom elements,
    /// allowing users to navigate to different routes within the frontend application.
    ///
    /// # Returns
    ///
    /// A boxed closure that can be attached as a mixin for a DOM element.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// let router = Router::<()>::new();
    /// let link_handler = router.link();
    ///
    /// // ... attach `link_handler` as an event handler to an anchor or button element ...
    /// ```
    pub fn link(&self) -> Box<dyn Fn(&Dom) + '_> {
        let router = self.clone();
        let cb = move |node: &Dom| {
            let router = router.clone();
            let handle_click = move |e: Event| {
                e.prevent_default();
                let element = e.current_target().unwrap().dyn_into::<Element>().unwrap();
                let href = element.get_attribute("href").unwrap();
                router.push(&href);
            };
            node.event("click", handle_click);
        };
        Box::new(cb)
    }

    /// Retrieves a signal for listening to route changes.
    ///
    /// This method returns a `MutableSignalCloned<String>` that can be used to listen for changes
    /// to the current route. It allows you to observe route changes and perform additional actions
    /// or updates in response to route navigation.
    ///
    /// # Returns
    ///
    /// A `MutableSignalCloned<String>` that represents the signal for route changes.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// let router = Router::<()>::new();
    /// let signal = router.signal();
    ///
    /// // ... use the `signal` to listen for route changes ...
    /// ```
    pub fn signal(&self) -> MutableSignalCloned<String> {
        self.current.signal_cloned()
    }

    /// Renders the appropriate content for the current route and appends it to the specified parent.
    ///
    /// This method is used internally to render the content associated with the current route and
    /// append it as a child to the provided `parent` DOM element (`DomType`). It is automatically
    /// called by the `mount` and `mount_to` methods of the `App` struct when mounting the frontend
    /// application.
    ///
    /// # Arguments
    ///
    /// * `app` - A reference to the `App<S>` instance to access the application state.
    /// * `parent` - A reference to the parent DOM element (`DomType`) where the content should be appended.
    ///
    /// # Returns
    ///
    /// A `Dom` element representing the rendered content for the current route.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// use hirola::dom::app::App;
    /// use hirola::dom::Dom;
    /// use hirola::prelude::*;
    /// #[derive(Clone)]
    /// struct AppState {
    ///     // ... fields and methods for your application state ...
    /// }
    ///
    /// fn home_page(app: &App<AppState>) -> Dom {
    ///     html! { <h1>"Home"</h1> }
    /// }
    ///
    /// fn about_page(app: &App<AppState>) -> Dom {
    ///     html! { <h1>"About"</h1> }
    /// }
    ///
    /// let mut app = App::new(AppState { /* ... */ });
    /// app.route("/", home_page);
    /// app.route("/about", about_page);
    /// let router = app.router().clone();
    /// let doc = web_sys::window().unwrap().document().unwrap();
    /// router.render(&app, &Dom::fragment());
    /// ```
    pub fn render(&self, app: &App<S>, parent: &Dom) -> Dom {
        let router = &self.handler;
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

        let current = self.current.clone();
        //Routing for navigating in history and escaping hash routes
        let handle_pop = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
            let path_name = web_sys::window().unwrap().location().pathname().unwrap();

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
                return;
            }
            current.set(path_name.to_string());
            log::debug!("pop handle : {path_name}");
        }) as Box<dyn Fn(_)>);

        web_sys::window()
            .unwrap()
            .set_onpopstate(Some(handle_pop.as_ref().unchecked_ref()));

        handle_pop.forget();
        let route = &self.current.clone();
        let router = router.clone();
        let app = app.clone();
        let node = parent.clone();
        let not_found = self.not_found.clone();
        let wait_for_next_route = route
            .signal_cloned()
            .map(move |route_match| {
                let match_result = router.at(&route_match);
                let page_fn = match match_result {
                    Ok(v) => v.value,
                    Err(_) => &not_found,
                };

                let builder = page_fn(&app);
                let dom = Dom::fragment();
                dom.append_child(&builder);
                node.replace_children_with(&dom);
                let window = web_sys::window().unwrap();
                window
                    .history()
                    .unwrap()
                    .push_state_with_url(&JsValue::default(), "", Some(&route_match))
                    .unwrap();
                log::debug!("Router received new path: {route_match}");
            })
            .to_future();
        parent.effect(wait_for_next_route);
        parent.clone()
    }

    /// Inserts a new route and its corresponding page rendering function into the router.
    ///
    /// This method registers a new route pattern and its associated page rendering function in the router.
    /// When a user navigates to the specified `path`, the corresponding `page` function will be called
    /// to render the content for that route.
    ///
    /// # Arguments
    ///
    /// * `path` - A string representing the route pattern to match. This can include path parameters
    ///            enclosed in curly braces, e.g., "/users/{id}".
    /// * `page` - A function that takes a reference to the `App<S>` instance and returns the rendered
    ///            DOM content (`Dom`). This function is responsible for generating the DOM structure
    ///            for the specified route.
    ///
    /// # Panics
    ///
    /// If the insertion into the router fails, this method will panic. However, in most cases, such a
    /// scenario is unlikely if there are no conflicts with existing routes or issues with the provided
    /// page rendering function.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// use hirola::prelude::*;
    /// use hirola::dom::Dom;
    /// use hirola_dom::app::App;
    ///
    /// // Define a custom function to render the home page
    /// fn home_page(_: &App<()>) -> Dom {
    ///     html! {
    ///         <h1>"Home"</h1>
    ///     }
    /// }
    ///
    /// // Create a new router and add a route for the home page
    /// let mut router = Router::<()>::new();
    /// router.insert("/", home_page);
    /// ```
    pub fn insert(&mut self, path: &str, page: fn(&App<S>) -> Dom) {
        self.handler.insert(path.to_string(), page).unwrap();
    }

    /// Sets the page rendering function for the not-found route.
    ///
    /// This method sets the page rendering function for the not-found route. When a user navigates to
    /// a route that does not match any registered paths in the router, the specified `page` function
    /// will be called to render the content for the not-found page.
    ///
    /// # Arguments
    ///
    /// * `page` - A function that takes a reference to the `App<S>` instance and returns the rendered
    ///            DOM content (`Dom`). This function is responsible for generating the DOM structure
    ///            for the not-found page.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// use hirola::dom::Dom;
    /// use hirola_dom::app::App;
    /// use hirola::prelude::*;
    /// // Define a custom function to render the not-found page
    /// fn not_found_page(_: &App<()>) -> Dom {
    ///     html! {
    ///         <h1>"Not Found"</h1>
    ///     }
    /// }
    ///
    /// // Create a new router and set the not-found page
    /// let mut router = Router::<()>::new();
    /// router.set_not_found(not_found_page);
    /// ```
    pub fn set_not_found(&mut self, page: fn(&App<S>) -> Dom) {
        self.not_found = Box::new(page);
    }

    /// Retrieves a clone of the route handler from the router.
    ///
    /// This method returns a clone of the route handler, which contains all the registered routes
    /// and their corresponding page rendering functions. The route handler is a part of the internal
    /// state of the router.
    ///
    /// # Returns
    ///
    /// A clone of the route handler, which is an instance of `matchit::Router<fn(&App<S>) -> Dom>`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use hirola::dom::app::router::Router;
    /// use hirola::dom::Dom;
    /// use hirola_dom::app::App;
    /// use hirola::prelude::*;
    ///
    /// // Define custom functions to render the home and about pages
    /// fn home_page(_: &App<()>) -> Dom {
    ///     html! {
    ///         <h1>"Home"</h1>
    ///     }
    /// }
    ///
    /// fn about_page(_: &App<()>) -> Dom {
    ///     html! {
    ///         <h1>"About"</h1>
    ///     }
    /// }
    ///
    /// // Create a new router and add routes for the home and about pages
    /// let mut router = Router::new();
    /// router.insert("/", home_page);
    /// router.insert("/about", about_page);
    ///
    /// // Get a clone of the route handler
    /// let cloned_handler = router.handler();
    /// ```
    pub fn handler(&self) -> matchit::Router<fn(&App<S>) -> Dom> {
        self.handler.clone()
    }
}

impl<S: Clone + 'static> Default for Router<S> {
    fn default() -> Self {
        Self::new()
    }
}
