use hirola::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::window;
use web_sys::HtmlInputElement;

#[derive(Clone)]
struct Todo {
    id: String,
    title: String,
    complete: bool,
}

fn TodoCard(
    todo: StateHandle<Todo>,
    on_remove: Box<dyn Fn() -> ()>,
    on_click: Box<dyn Fn() -> ()>,
) -> TemplateResult<DomNode> {
    html! {
        <div class="flex mb-4 items-center">
            <p class="w-full text-grey-darkest">{todo.get().title.clone()}</p>
            <button
                on:click={move |e| {
                    on_click();
                }}
                class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">
                "View"
            </button>
            <button
                on:click={move |e| {
                    on_remove()
                }}
                class="flex-no-shrink p-2 ml-2 border-2 rounded text-red border-red hover:text-white hover:bg-red">
                "Remove"
            </button>
        </div>
    }
}

fn todo_view(app: &HirolaApp) -> TemplateResult<DomNode> {
    let router = app.data::<Router>().unwrap().clone();
    let route = router.params().get();
    let param = route.params.get("id").unwrap_or(&"1".to_string()).clone();
    let todo = app
        .data::<TodoStore>()
        .unwrap()
        .clone()
        .todos
        .to_vec()
        .iter()
        .find(|s| s.get().id == param)
        .unwrap()
        .clone();
    html! {
        <div class="h-100 w-full flex items-center justify-center bg-teal-lightest font-sans">
            <div class="bg-white rounded shadow p-6 m-4 w-full lg:w-3/4 lg:max-w-lg">
                <h1 class="text-grey-darkest">{todo.get().title.clone()}</h1>
                <a href="/">"Back Home"</a>
            </div>
        </div>
    }
}

fn home(app: &HirolaApp) -> TemplateResult<DomNode> {
    let router = app.data::<Router>().unwrap().clone();

    let state = app.data::<TodoStore>().unwrap().clone().todos;
    let st1 = state.clone();
    let add_new = move |e: Event| {
        let input = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("add")
            .unwrap();
        let input = input.dyn_ref::<HtmlInputElement>().unwrap();
        st1.push(Signal::new(Todo {
            id: format!("{}", st1.to_vec().len() + 1),
            title: input.value(),
            complete: false,
        }))
    };

    let st = state.clone();

    let todos: TemplateList<DomNode> = state
        .map(move |todo| {
            let id = todo.get().id.clone();
            let title = todo.get().title.clone();
            let st = st.clone();
            let router = router.clone();
            let cb = Box::new(move || {
                let index = st
                    .to_vec()
                    .iter()
                    .position(|t| t.get().title == title)
                    .unwrap();
                st.remove(index);
            });
            html! {
                <TodoCard
                    todo=todo.handle()
                    on_remove=cb
                    on_click={Box::new(move || {
                        router.push(&format!("/todo/{}", id));
                    })}
                />
            }
        })
        .template_list();

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
                        <button class="flex-no-shrink p-2 border-2 rounded text-teal border-teal hover:text-white hover:bg-teal" on:click={add_new}>"Add"</button>
                    </div>
                </div>
                <div>
                {todos.clone()}
                </div>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct TodoStore {
    todos: SignalVec<Signal<Todo>>,
}

fn main() {
    let todos = vec![
        Signal::new(Todo {
            id: String::from("1"),
            title: String::from("Add another component to Tailwind Components"),
            complete: false,
        }),
        Signal::new(Todo {
            id: String::from("2"),
            title: String::from("Submit Todo App Component to Tailwind Components"),
            complete: true,
        }),
    ];
    let todos = SignalVec::with_values(todos);

    let mut app = HirolaApp::new();
    app.extend(TodoStore { todos });

    let mut router = Router::new();
    router.add("/todo/:id", todo_view);
    router.add("/", home);

    app.mount("body", router);
}
