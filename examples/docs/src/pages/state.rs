use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use crate::App;
use hirola::prelude::*;

pub fn state_page(_app: &App) -> Dom {
    html! {
        <div>
            <SeoTitle title={"State Management | Hirola"} />

            <h1>"State Management"</h1>
            <p>"Hirola is un-opinionated in state management. It should be pretty easy to roll out your own."</p>
            <h2>"Getting started"</h2>
            <CodePreview
             code=
r#"let window = web_sys::window().unwrap();
let document = window.document().unwrap();
let body = document.body().unwrap();

struct App {
    todos: MutableVec<Todo>,
}

impl App {
    ///...
}

let mut app = App::new();

app.mount(&body, |app| {
    let todos: &MutableVec<Todo> = app.todos;
    html! {
        ...
    }
});
"#
            file="src/main.rs"
            />
        </div>
    }
}
