use hirola::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlInputElement;

#[derive(Clone, PartialEq)]
struct Todo {
    id: String,
    title: String,
    complete: bool,
}

#[component]
fn TodoCard(todo: ReadOnlyMutable<Todo>, router: Router, todos: Mutable<Vec<Mutable<Todo>>>) {
    let todo = (&*todo).clone().get();
    let id = todo.id.clone();
    let href = format!("/todo/{}", id);

    let title = todo.title.clone();
    let tl = title.clone();
    let on_remove = todos.update(move |todos, _e| {
        let index = todos
            .get()
            .iter()
            .position(|t| t.get().title == tl.clone())
            .unwrap();
        todos.remove(index);
    });
    html! {
        <div class="flex mb-4 items-center">
            <p class="w-full text-grey-darkest">{title.clone()}</p>
            <a
                href=href
                class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green"
                mixin:route=&router.link()
                >
                "View"
            </a>
            <button
                on:click=on_remove
                class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">
                "Remove"
            </button>
        </div>
    }
}

fn todo_view(app: &App<S, G>) -> Dom {
    let router = app.data::<Router>().unwrap().clone();
    let route = router.params().get();
    let param = route.params.get("id").unwrap_or(&"1".to_string()).clone();
    let todo = app
        .data::<TodoStore>()
        .unwrap()
        .clone()
        .todos
        .get_untracked()
        .iter()
        .find(|s| s.get().id == param)
        .unwrap()
        .clone();
    html! {
        <div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
            <div class="bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
                <h1 class="text-grey-darkest">{todo.get().title.clone()}</h1>
                <a mixin:route=&router.link() href="/">"Back Home"</a>
            </div>
        </div>
    }
}

fn home(app: &App<S, G>) -> Dom {
    let router = app.data::<Router>().unwrap().clone();

    let state = app.data::<TodoStore>().unwrap().clone().todos;

    let add_new = state.callback(|todos, _e| {
        let input = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("add")
            .unwrap();
        let input = input.dyn_ref::<HtmlInputElement>().unwrap();
        todos.push(Mutable::new(Todo {
            id: format!("{}", todos.get_untracked().len() + 1),
            title: input.value(),
            complete: false,
        }))
    });

    html! {
        <div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
            <div class="bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
                <div class="mb-4">
                    <h1 class="text-grey-darkest">"Todo List"</h1>
                    <div class="flex mt-4">
                        <input
                            id="add"
                            class="shadow appearance-none border rounded w-full py-2 px-3 mr-4 text-grey-darker"
                            placeholder="Add Todo"/>
                        <button
                            class="flex-no-shrink p-2 border-2 rounded text-teal border-teal hover:text-white hover:bg-teal"
                            on:click=add_new>
                            "Add"
                        </button>
                    </div>
                </div>
                <div>
                <Keyed
                    props={
                        KeyedProps {
                            iterable: state.handle(),
                            template: move | todo | {
                                // let id = todo.get().id.clone();
                                // let title = todo.get().title.clone();

                                html! {
                                    <>
                                        <TodoCard
                                            todo=todo.handle()
                                            router=router.clone()
                                            todos=state.clone()
                                        />
                                    </>
                                }
                            },
                            key: |item| (*item).clone().get().title.clone()
                        }
                    }
                    />
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct TodoStore {
    todos: Mutable<Vec<Mutable<Todo>>>,
}

fn index(app: &App<S, G>) -> Dom {
    let router = app.data::<Router>().unwrap().clone();
    let app = app.clone();
    html! {
        <div>
            {router.render(&app)}
        </div>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let todos = vec![
        Mutable::new(Todo {
            id: String::from("1"),
            title: String::from("Add another component to Tailwind Components"),
            complete: false,
        }),
        Mutable::new(Todo {
            id: String::from("2"),
            title: String::from("Submit Todo App Component to Tailwind Components"),
            complete: true,
        }),
    ];
    let todos = Mutable::new(todos);

    let mut app = App<S, G>::new();

    let mut router = Router::new();
    router.route("/", home);
    router.route("/todo/:id", todo_view);

    app.extend(TodoStore { todos });
    app.extend(router);

    app.mount(&body, index);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn app_renders() {
        let mut router = Router::new();
        router.route("/", |app| {
            html! {
                <p>"Homepage"</p>
            }
        });
    }
}
