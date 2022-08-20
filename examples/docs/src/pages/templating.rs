use crate::components::seo_title::SeoTitle;
use crate::pages::CodePreview;
use hirola::prelude::*;

pub fn templating_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
          <SeoTitle title={"Templating | Hirola"} />
            <h1>"Templating"</h1>
            <p>"Hirola uses rsx which is an implementation of jsx in rust. This also means it inherits all the caveats."</p>
            <h2>"Iteration"</h2>
            <p>"Looping through an array of values is important to any framework. Frameworks like react provide a key mechanism to improve on the rerendering."</p>
            <h3>"Keyed"</h3>
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
