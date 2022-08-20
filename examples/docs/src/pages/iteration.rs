use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn iteration_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <h1>"Iteration"</h1>
            <p>"Looping through an array of values is important to any framework. Frameworks like react provide a key mechanism to improve on the rerendering."</p>
            <h2>"Keyed"</h2>
            <CodePreview
             code=
"<Keyed
    props={
        KeyedProps {
        iterable: todos.handle(),
        template: move | todo | {
            html! {
                <TodoCard
                    todo=todo.handle()
                />
            }
        },
        key: |item| item.get().title
        }
    }
/>
"
            file_name="src/main.rs"
            />
        </div>
    }
}
