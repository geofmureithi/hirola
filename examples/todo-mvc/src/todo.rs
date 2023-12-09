use crate::{todo::util::trim, visible, Route, State};
use hirola::{
    dom::{app::App, Dom},
    prelude::*,
    signal::{Mutable, Signal, SignalExt},
};
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    id: u32,
    title: Mutable<String>,
    pub completed: Mutable<bool>,

    #[serde(skip)]
    editing: Mutable<bool>,
}

impl Todo {
    pub fn new(id: u32, title: String) -> Arc<Self> {
        Arc::new(Self {
            id,
            title: Mutable::new(title),
            completed: Mutable::new(false),
            editing: Mutable::new(false),
        })
    }

    fn set_completed(&self, completed: bool) {
        self.completed.set_neq(completed);
    }

    fn remove(&self, app: &App<State>) {
        app.state().remove_todo(self);
        app.state().serialize();
    }

    fn is_visible(&self, app: &App<State>) -> impl Signal<Item = bool> {
        (map_ref! {
            let route = app.router().signal(),
            let completed = self.completed.signal() =>
            match Route::from_str(route).unwrap() {
                Route::Active => !completed,
                Route::Completed => *completed,
                Route::All => true,
            }
        })
        .dedupe()
    }

    fn is_editing(&self) -> impl Signal<Item = bool> {
        self.editing.signal().dedupe()
    }

    fn cancel_editing(&self) {
        self.editing.set_neq(false);
    }

    fn done_editing(&self) {
        self.editing.set_neq(false);
    }

    pub fn render(todo: Arc<Self>, app: &App<State>) -> Dom {
        fn is_checked(event: &Event) -> bool {
            event
                .target()
                .unwrap()
                .dyn_ref::<HtmlInputElement>()
                .unwrap()
                .checked()
        }
        let toggle_edit = todo.callback_with(|todo, _| todo.editing.set_neq(true));

        let title = todo.title.clone();
        let todo_class = (map_ref! {
                    let is_editing = todo.is_editing(),
                    let is_completed = todo.completed.signal() =>
                    match (is_editing, is_completed) {
                        (true, true) => "editing completed",
                        (true, false) => "editing",
                        (false, true) => "completed",
                        (false, false) => "",
        }
                })
        .dedupe();
        let app = app.clone();
        let state = app.state().clone();
        let is_visible = todo.is_visible(&app);

        let handle_edit: Box<dyn Fn(Event)> = todo.callback_with(move |todo, event| {
            let target = event.target().unwrap();
            let input = target.dyn_ref::<HtmlInputElement>().unwrap();
            if event.dyn_ref::<KeyboardEvent>().unwrap().key() == "Enter" {
                event.prevent_default();
                if let Some(title) = trim(&input.value()) {
                    todo.title.set(title.to_owned());
                } else {
                    state.remove_todo(todo);
                }
                input.blur().unwrap();
                todo.done_editing();
                state.serialize();
            }
            if event.dyn_ref::<KeyboardEvent>().unwrap().key() == "Escape" {
                event.prevent_default();
                todo.cancel_editing();
            }
        });
        let todo_clone = todo.clone();

        html! {
            <li mixin:identity=visible(is_visible) bind:class=todo_class>
                <div class="view">
                    {if todo.completed.signal() as Signal {
                        html!{
                            <input
                                class="toggle"
                                type="checkbox"
                                checked=""
                                on:change={todo_clone.callback_with(move |todo, event| {
                                    todo.set_completed(is_checked(&event));
                                })}
                            />
                        }
                    } else {
                        html!{
                            <input
                                class="toggle"
                                type="checkbox"
                                on:change={todo_clone.callback_with(move |todo, event| {
                                    todo.set_completed(is_checked(&event));
                                })}
                            />
                        }
                    }}

                    <label on:dblclick=toggle_edit>{title}</label>
                    <button class="destroy" on:click=todo.callback_with(move|todo, _| todo.remove(&app))/>
                </div>
                <input class="edit" on:keydown=handle_edit bind:value=todo.title.signal_cloned() />
            </li>
        }
    }
}

impl PartialEq<Todo> for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Callback<web_sys::Event> for Todo {}

pub mod util {
    use web_sys::{window, Storage};

    pub fn local_storage() -> Storage {
        window().unwrap().local_storage().unwrap().unwrap()
    }

    #[inline]
    pub fn trim(input: &str) -> Option<&str> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    }
}
