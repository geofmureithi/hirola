use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn state_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <SeoTitle title={"State Management | Hirola"} />

            <h1>"State Management"</h1>
            <p>"Hirola is un-opinionated in state management. It should be pretty easy to roll out your own. To enable the global state management use the feature flag "<code class="one-liner">"global-state"</code></p>
            <h2>"Getting started"</h2>
            <CodePreview
             code=
r#"let mut app = HirolaApp::new();

let todos = Signal::new(todos);
app.extend(TodoStore { todos });

app.mount("body", |app| {
    let todos: &TodoStore = app.data().unwrap();
    html! {
        ...
    }
});
"#
            file_name="src/main.rs"
            />
        </div>
    }
}
