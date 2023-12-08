pub mod router;
use router::Router;
use std::fmt::Debug;

use crate::Dom;

#[derive(Debug, Clone)]
pub struct App<S: 'static> {
    router: Router<S>,
    state: S,
}

/// The main application struct for the frontend app.
///
/// This struct represents the core of the frontend application and holds the application state
/// as well as the routing information. It is parameterized over the state type `S`, which should
/// be clone-able and 'static to ensure proper lifetime management. The `App` struct is created
/// using the `new` method, which takes an initial state `S` and returns a new instance of the `App`.
///
/// # Example
/// ```no_run
/// use hirola::prelude::*;
/// use hirola_dom::app::App;
/// #[derive(Clone)]
/// struct AppState {
///     // ... fields and methods for your application state ...
/// }
///
/// fn main() {
///     let initial_state = AppState { /* ... */ };
///     let app = App::new(initial_state);
/// }
/// ```
impl<S: Clone + 'static> App<S> {
    /// Creates a new instance of the App with the given initial state.
    ///
    /// # Arguments
    ///
    /// * `state` - The initial state for the application.
    ///
    /// # Returns
    ///
    /// A new instance of `App<S>`.
    pub fn new(state: S) -> Self {
        Self {
            state,
            router: Router::new(),
        }
    }

    /// Get a reference to the current application state.
    ///
    /// # Returns
    ///
    /// A reference to the application state of type `S`.
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Get a reference to the router associated with the application.
    ///
    /// # Returns
    ///
    /// A reference to the `Router<S>` instance responsible for handling routing in the app.
    pub fn router(&self) -> &Router<S> {
        &self.router
    }

    /// Add a new route to the application.
    ///
    /// # Arguments
    ///
    /// * `path` - The path for the new route, a string representing the route pattern.
    /// * `page` - A function that takes a reference to the `App<S>` and returns a `Dom` element.
    ///
    /// # Example
    /// ```no_run
    /// use hirola::prelude::*;
    /// use hirola::dom::app::App;
    /// use hirola::dom::Dom;
    /// 
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
    /// ```
    pub fn route(&mut self, path: impl AsRef<str>, page: fn(&Self) -> Dom) {
        self.router.handler.insert(path.as_ref().to_string(), page).unwrap();
    }

    /// Set the not-found page for the application.
    ///
    /// This page will be displayed when the requested route does not match any registered routes.
    ///
    /// # Arguments
    ///
    /// * `page` - A function that takes a reference to the `App<S>` and returns a `Dom` element.
    ///
    /// # Example
    /// ```no_run
    /// use hirola::prelude::*;
    /// use hirola::dom::app::App;
    /// use hirola::dom::Dom;
    /// 
    /// #[derive(Clone)]
    /// struct AppState {
    ///     // ... fields and methods for your application state ...
    /// }
    ///
    /// fn not_found_page(app: &App<AppState>) -> Dom {
    ///     html! { <h1>"Not Found"</h1> }
    /// }
    ///
    /// let mut app = App::new(AppState { /* ... */ });
    /// app.set_not_found(not_found_page);
    /// ```
    pub fn set_not_found(&mut self, page: fn(&Self) -> Dom) {
        self.router.set_not_found(page);
    }
}

impl<S: Clone + 'static> App<S> {
    /// Mounts the application on the web page body and starts the rendering process.
    ///
    /// This method should be called after setting up all the routes and configuring the application.
    /// It mounts the application on the web page body, rendering the appropriate page based on the
    /// current route. The rendering process will be managed by the `Router` associated with the app.
    ///
    /// # Panics
    ///
    /// This method will panic if it fails to access the `window` or `document` objects from the
    /// `web_sys` module. Make sure to run the application in a browser environment with WebAssembly
    /// support to avoid panics.
    ///
    /// # Example
    ///
    /// ```no_run
    /// fn main() {
    ///     use hirola::prelude::*;
    ///     use hirola::dom::app::App;
    ///     #[derive(Clone)]
    ///     struct AppState {
    ///         // ... fields and methods for your application state ...
    ///     }
    ///     let initial_state = AppState { /* ... */ };
    ///     let app = App::new(initial_state);
    ///     
    ///     // ... add routes and set up the app ...
    ///     
    ///     // Mount the app on the web page body and start rendering
    ///     app.mount();
    /// }
    /// ```
    pub fn mount(&self) {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let router = self.router.clone();
        let dom = router.render(
            self,
            &crate::Dom {
                node: document.body().unwrap().into(),
                ..Default::default()
            },
        );
        // We leak the root node to avoid callbacks and futures being dropped
        std::mem::forget(dom);
    }

    /// Mounts the application on a specified parent node and starts the rendering process.
    ///
    /// This method should be called after setting up all the routes and configuring the application.
    /// It mounts the application on the provided parent node, rendering the appropriate page based on
    /// the current route. The rendering process will be managed by the `Router` associated with the app.
    ///
    /// # Arguments
    ///
    /// * `parent` - The web_sys::Node to which the application should be mounted.
    ///
    /// # Example
    ///
    /// ```no_run
    /// fn main() {
    ///     use hirola::prelude::*;
    ///     use hirola::dom::app::App;
    ///     #[derive(Clone)]
    ///     struct AppState {
    ///         // ... fields and methods for your application state ...
    ///     }
    ///     let app = App::new(AppState { });
    ///     // ... add routes and set up the app ...
    ///     
    ///     // Find the parent node where the app should be mounted
    ///     let parent_node = web_sys::window()
    ///         .unwrap()
    ///         .document()
    ///         .unwrap()
    ///         .get_element_by_id("app-container")
    ///         .unwrap();
    ///
    ///     // Mount the app on the specified parent node and start rendering
    ///     app.mount_to(&parent_node);
    /// }
    /// ```
    pub fn mount_to(&self, parent: &web_sys::Node) {
        let router = self.router.clone();
        let dom = router.render(
            self,
            &crate::Dom {
                node: parent.clone(),
                ..Default::default()
            },
        );
        // We leak the root node to avoid callbacks and futures being dropped
        std::mem::forget(dom);
    }

    /// Mounts the application on a specified parent node and starts the rendering process.
    ///
    /// This method should be called after setting up all the routes and configuring the application.
    /// It mounts the application on the provided parent node, rendering the appropriate page based on
    /// the current route. The rendering process will be managed by the `Router` associated with the app.
    ///
    /// # Arguments
    ///
    /// * `parent` - The `web_sys::Node` to which the application should be mounted.
    /// * `cb` - A callback function that takes the generated `Dom` element representing the rendered
    ///          content as input and returns a modified `Dom` element. This callback can be used to
    ///          wrap the rendered content with layout components or apply any additional transformations.
    ///
    /// # Example
    ///
    /// ```no_run
    /// fn main() {
    ///     use hirola::prelude::*;
    ///     use hirola::dom::app::App;
    ///     use hirola::dom::Dom;
    ///
    ///     #[derive(Clone)]
    ///     struct AppState {
    ///         // ... fields and methods for your application state ...
    ///     }
    ///     let app = App::new(AppState { });
    ///     // ... add routes and set up the app ...
    ///     
    ///     // Find the parent node where the app should be mounted
    ///     let parent_node = web_sys::window()
    ///         .unwrap()
    ///         .document()
    ///         .unwrap()
    ///         .get_element_by_id("app-container")
    ///         .unwrap();
    ///
    ///     // Mount the app on the specified parent node and start rendering
    ///     // In this example, we wrap the rendered content with a layout component
    ///     app.mount_with(&parent_node, |app| {
    ///         let router = app.router().clone();
    ///         let inner = router.render(app, &Dom::new_from_node(&parent_node));
    ///         html! {
    ///             <main>
    ///                <nav>
    ///                     <ul>
    ///                         <li>"Home"</li>
    ///                         <li>"About"</li>
    ///                     </ul>
    ///                 </nav>
    ///                 <main>
    ///                 {inner}
    ///                 </main>
    ///             </main>
    ///         }
    ///     });
    /// }
    /// ```
    pub fn mount_with(&self, parent: &web_sys::Node, cb: impl Fn(&Self) -> Dom) {
        let res = cb(self);
        parent.append_child(&res.inner_element()).unwrap();
        // We leak the root node to avoid callbacks and futures being dropped
        std::mem::forget(res);
    }
}
