use hirola::prelude::*;

#[derive(Clone)]
struct Todo {
    title: String,
    complete: bool,
}

fn TodoCard(todo: StateHandle<Todo>, on_remove: Box<dyn Fn() -> ()>) -> TemplateResult<DomNode> {
    html! {
        <div class="flex mb-4 items-center">
            <p class="w-full text-grey-darkest">{todo.get().title.clone()}</p>
            <button
                class="flex-no-shrink p-2 ml-4 mr-2 border-2 rounded hover:text-white text-green border-green hover:bg-green">
                "Done"
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

fn TodoView(route: RouteMatch) -> TemplateResult<DomNode> {
    let param = route.params.get("id").unwrap().clone();
    let display = format!("Displaying ---> {}", param);
    html! {
        <div>
            <h1 class="text-grey-darkest">"Todo View"</h1>
            <a href={format!("/todo/{param}")}>{display.clone()}</a>
        </div>
    }
}

fn Home(route: RouteMatch) -> TemplateResult<DomNode> {
    let todos = vec![
        Signal::new(Todo {
            title: String::from("Add another component to Tailwind Components"),
            complete: false,
        }),
        Signal::new(Todo {
            title: String::from("Submit Todo App Component to Tailwind Components"),
            complete: true,
        }),
    ];
    let state = SignalVec::with_values(todos);
    let st1 = state.clone();
    let add_new = move |e: Event| {
        st1.push(Signal::new(Todo {
            title: String::from("Test Test Test"),
            complete: false,
        }))
    };

    let st = state.clone();

    let todos: TemplateList<DomNode> = state
        .map(move |todo| {
            let title = todo.get().title.clone();
            let st = st.clone();
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
                    todo={todo.handle()}
                    on_remove={cb}
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
                        <input class="shadow appearance-none border rounded w-full py-2 px-3 mr-4 text-grey-darker" placeholder="Add Todo"/>
                        <button class="flex-no-shrink p-2 border-2 rounded text-teal border-teal hover:text-white hover:bg-teal" on:click={add_new}>"Add"</button>
                    </div>
                </div>
                <div>
                {todos.clone()}
                </div>
                <a href="/todo/9">"Go to 9"</a>
            </div>
        </div>
    }
}

fn App() -> TemplateResult<DomNode> {
    let mut router = Router::new();
    router.add("/todo/:id", TodoView);
    router.add("/", Home);
    router.render()
}

fn main() {
    let app = HirolaApp::new();

    app.mount("body", App);
}
