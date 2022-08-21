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
                <h2>"Basic"</h2>
                <p>"If you are iterating over a non-signal iterator, you can use the normal for-loop"</p>
                <h3>"Example"</h3>
                <CodePreview
                  code=
"{for i in 0..5 {
  html! {
      <ul>
          <li>{i}</li>
      </ul>
  }
}}
"
    file_name="src/main.rs"
    />
              <div class="demo">
              {for i in 0..5 {
                html! {
                    <ul>
                        <li>{i}</li>
                    </ul>
                }
              }}
              </div>
                <h2>"With Signal"</h2>
                <p>"Sometimes, you are working with a signal and want to react to changes on the ui. You can use Keyed and Indexed"</p>
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
            <h3>"Indexed"</h3>
            <CodePreview
            code=
"<Indexed
  props={
  IndexedProps {
    iterable: todos.handle(),
    template: move | todo | {
        html! {
            <TodoCard
                todo=todo.handle()
            />
        }
    },
  }
  }
/>
"
           file_name="src/main.rs"
           />
           <h2>"Components"</h2>
           <p>"One can write components as functions starting with uppercase"</p>
           <CodePreview
            code="fn Todo(router: Router) -> Dom {}
html! {
  <Todo router={router} />
}
"
        file_name="src/main.rs"
           />
            </div>
        }
}
