mod todo;

use hirola::dom::app::App;
use hirola::dom::mixins::text;
use hirola::dom::Dom;
use hirola::prelude::*;
use hirola::signal::{Mutable, Signal, SignalExt};
use hirola::signal_vec::{MutableVec, SignalVec, SignalVecExt};
use serde::{Deserialize, Serialize};
use std::cell::Cell;
use std::sync::Arc;
use strum::{AsRefStr, EnumString};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, HtmlInputElement, KeyboardEvent};

use crate::todo::util::{local_storage, trim};
use crate::todo::Todo;

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsRefStr, EnumString)]
pub enum Route {
    #[strum(serialize = "/active")]
    Active,
    #[strum(serialize = "/completed")]
    Completed,
    #[strum(serialize = "/")]
    All,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]

enum Mode {
    #[default]
    AddNew,
    Editing(Todo),
}

impl Callback<web_sys::Event> for State {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct State {
    todo_id: Cell<u32>,

    #[serde(skip)]
    mode: Mutable<Mode>,

    todo_list: MutableVec<Arc<Todo>>,
}

impl State {
    fn new() -> Arc<Self> {
        Arc::new(State {
            todo_id: Cell::new(0),
            mode: Mutable::new(Mode::AddNew),
            todo_list: MutableVec::new(),
        })
    }

    pub fn deserialize() -> Arc<Self> {
        local_storage()
            .get_item("todos-hirola")
            .unwrap()
            .and_then(|state_json| serde_json::from_str(state_json.as_str()).ok())
            .unwrap_or_else(State::new)
    }

    pub fn serialize(&self) {
        let state_json = serde_json::to_string(self).unwrap();

        local_storage()
            .set_item("todos-hirola", state_json.as_str())
            .unwrap();
    }

    fn create_new_todo(&self, title: &str) {
        // let mut title = self.new_todo_title.lock_mut();

        // Only create a new Todo if the text box is not empty
        if let Some(trimmed) = trim(&title) {
            let id = self.todo_id.get();
            self.todo_id.set(id + 1);

            self.todo_list
                .lock_mut()
                .push_cloned(Todo::new(id, trimmed.to_string()));

            self.serialize();
        }
    }

    pub fn remove_todo(&self, todo: &Todo) {
        self.todo_list.lock_mut().retain(|x| **x != *todo);
    }

    fn remove_all_completed_todos(&self) {
        self.todo_list
            .lock_mut()
            .retain(|todo| todo.completed.get() == false);
    }

    fn set_all_todos_completed(&self, checked: bool) {
        for todo in self.todo_list.lock_ref().iter() {
            todo.completed.set_neq(checked);
        }

        self.serialize();
    }

    fn completed(&self) -> impl SignalVec<Item = bool> {
        self.todo_list
            .signal_vec_cloned()
            .map_signal(|todo| todo.completed.signal())
    }

    fn completed_len(&self) -> impl Signal<Item = usize> {
        self.completed().filter(|completed| *completed).len()
    }

    fn not_completed_len(&self) -> impl Signal<Item = usize> {
        self.completed().filter(|completed| !completed).len()
    }

    fn has_todos(&self) -> impl Signal<Item = bool> {
        self.todo_list
            .signal_vec_cloned()
            .len()
            .map(|len| len > 0)
            .dedupe()
    }

    // fn render_button(app: &State, text: &str, route: Route) -> Dom {
    //     html!("li", {
    //         .children(&mut [
    //             link!(route.to_url(), {
    //                 .text(text)
    //                 .class_signal("selected", app.route().map(move |x| x == route))
    //             })
    //         ])
    //     })
    // }

    // fn render_footer(app: Arc<Self>) -> Dom {

    // pub fn render(app: Arc<Self>) -> Dom {
    //     html!("section", {
    //         .class("todoapp")

    //         // Update the Route when the URL changes
    //         .future(routing::url()
    //             .signal_ref(|url| Route::from_url(url))
    //             .for_each(clone!(app => move |route| {
    //                 app.route.set_neq(route);
    //                 async {}
    //             })))

    //         .children(&mut [
    //             Self::render_header(app.clone()),
    //             Self::render_main(app.clone()),
    //             Self::render_footer(app.clone()),
    //         ])
    //     })
    // }
}

#[component]
fn Header(app: App<Arc<State>>) -> Dom {
    let state = app.state().clone();
    let create_new: Box<dyn Fn(Event)> = Box::new(move |event| {
        let target = event.target().unwrap();
        let input = target.dyn_ref::<HtmlInputElement>().unwrap();
        if event.dyn_ref::<KeyboardEvent>().unwrap().key() == "Enter" {
            event.prevent_default();
            app.state().create_new_todo(&input.value());
            input.set_value("");
        }
    });
    html! {
        <header class="header">
            <h1>"Todos"</h1>
            <input
                focus=true
                class="new-todo"
                placeholder="What needs to be done?"
                bind:value=state.mode.signal_cloned().map(|_|"")
                on:keydown=create_new
            />
        </header>
    }
}

#[component]
fn Button<'a>(app: App<Arc<State>>, text: &'a str, route: Route) -> Dom {
    let router = app.router().signal();
    html! {
        <li>
            <a
                x:identity=app.router().link()
                bind:class=router.map(move |x| x == route.as_ref()).dedupe_map(|b| if *b {"selected"} else {""})
                href=route.as_ref()>{text}</a>
        </li>
    }
}

#[component]
fn Footer(app: App<Arc<State>>) -> Dom {
    let clear_completed = app.state().callback_with(move |state, e| {
        state.remove_all_completed_todos();
        state.serialize();
        e.prevent_default();
    });
    let count = app.state().not_completed_len().map(|len| len.to_string());
    let left_text = app.state().not_completed_len().map(|len| {
        if len == 1 {
            " item left"
        } else {
            " items left"
        }
    });
    let todo_count = (map_ref! {
        let cnt = count,
        let left_text = left_text =>
        format!("{cnt}{left_text}")
    })
    .dedupe_cloned();

    let has_todos = app.state().has_todos();
    html! { <footer
                bind:value=app.state().mode.signal_cloned().map(|_|"")
                mixin:identity=visible(has_todos)
                class="footer">
                <span class="todo-count">
                    <strong x:identity={text(todo_count)}></strong>
                </span>
                <ul class="filters">
                    <Button app={app.clone()} text="All" route={Route::All} />
                    <Button app={app.clone()} text="Active" route={Route::Active} />
                    <Button app={app.clone()} text="Completed" route={Route::Completed} />
                </ul>
                <button on:click=clear_completed mixin:identity=visible(app.state().completed_len().map(|len| len > 0).dedupe()) class="clear-completed">
                    "Clear completed"
                </button>
        </footer>
    }

    //     html!("footer", {
    //         .class("footer")

    //         .visible_signal(app.has_todos())

    //         .children(&mut [
    //             html!("span", {
    //                 .class("todo-count")

    //                 .children(&mut [
    //                     html!("strong", {
    //                         .text_signal(app.not_completed_len().map(|len| len.to_string()))
    //                     }),

    //                     text_signal(app.not_completed_len().map(|len| {
    //                         if len == 1 {
    //                             " item left"
    //                         } else {
    //                             " items left"
    //                         }
    //                     })),
    //                 ])
    //             }),

    //             html!("ul", {
    //                 .class("filters")
    //                 .children(&mut [
    //                     Self::render_button(&app, "All", Route::All),
    //                     Self::render_button(&app, "Active", Route::Active),
    //                     Self::render_button(&app, "Completed", Route::Completed),
    //                 ])
    //             }),

    //             html!("button", {
    //                 .class("clear-completed")

    //                 // Show if there is at least one completed item.
    //                 .visible_signal(app.completed_len().map(|len| len > 0).dedupe())

    //                 .event(clone!(app => move |_: events::Click| {
    //                     app.remove_all_completed_todos();
    //                     app.serialize();
    //                 }))

    //                 .text("Clear completed")
    //             }),
    //         ])
    //     })
    // }
}

#[component]
fn Main(app: App<Arc<State>>) -> Dom {
    let state = app.state().clone();
    let has_todos = state.has_todos();
    let on_toggle: Box<dyn Fn(Event)> = Box::new(move |event| {
        let target = event.target().unwrap();
        let input = target.dyn_ref::<HtmlInputElement>().unwrap();
        state.set_all_todos_completed(input.checked())
    });
    html! {
        <section mixin:identity=visible(has_todos) class="main">
            <input
                class="toggle-all"
                id="toggle-all"
                type="checkbox"
                bind:checked=app.state().not_completed_len().map(|len| len == 0)
                on:change=on_toggle
            />
            <label for="toggle-all">"Mark all as complete"</label>
            <ul class="todo-list">
                {app.state().todo_list.signal_vec_cloned().map_render(move |todo| Todo::render(todo, &app))}
            </ul>
        </section>
    }
}
pub fn visible<S>(signal: S) -> Box<dyn FnOnce(&Dom)>
where
    S: Signal<Item = bool> + SignalExt + 'static,
{
    let cb = move |node: &Dom| {
        let element = node.inner_element().dyn_into::<HtmlElement>().unwrap();
        let default = element
            .style()
            .get_property_value("display")
            .unwrap_or("block".to_owned());

        let fut = signal
            .dedupe_map(move |val| {
                if *val {
                    element.style().set_property("display", &default).unwrap();
                } else {
                    element.style().set_property("display", "none").unwrap();
                }
            })
            .to_future();
        node.effect(fut);
    };
    Box::new(cb)
}

fn page(app: &App<Arc<State>>) -> Dom {
    html! {
        <div id="app" class="todoapp">
            <Header app=app.clone()/>
            <Main app=app.clone()/>
            <Footer app=app.clone()/>
        </div>
    }
}

fn main() {
    let mut app = App::new(State::deserialize());
    app.route(Route::All, page);
    app.route(Route::Completed, page);
    app.route(Route::Active, page);
    app.mount();
}
