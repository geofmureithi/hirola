use crate::components::seo_title::SeoTitle;
use crate::pages::CodePreview;
use hirola::prelude::*;

pub fn templating_page(_app: &HirolaApp) -> Dom {
    html! {
            <div>
              <SeoTitle title={"Templating | Hirola"} />
                <h1>"Templating"</h1>
                <p>"Hirola uses rsx which is an implementation of jsx in rust. This also means it inherits all the caveats."</p>

                <br/>
                <h1>"Iteration"</h1>
                <p>"Looping through an array of values is important to any framework. Frameworks like react provide a key mechanism to improve on the rerendering."</p>
                <h2>"Non-Reactive Looping"</h2>
                <p>"If you are iterating over a non-signal iterator, you can use the normal for-loop pattern."</p>
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
    file="src/main.rs"
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

              <br/>
                <h2>"Reactive Looping"</h2>
                <p>"However, if you are looping something that are reactive, "
                  <code class="one-liner">
                  "Keyed"
                  </code>
                " and "
                  <code class="one-liner">
                  "Indexed"
                  </code>
                " will be your solution."
                </p>

                <br/>
                <h3>"Keyed"</h3>
                <p>
                "You can look at"
                  <code class="one-liner">
                  "Keyed"
                  </code>
                " just like any other components. It takes 1 prop," <code class="one-liner">"KeyedProp"</code>", which has 3 fields in it: " <code class="one-liner">"iterator"</code>", "<code class="one-liner">"template"</code>", and " <code class="one-liner">"key"</code>
                "."
                </p>
                <p>
                  <code class="one-liner">
                  "iterable"
                  </code>
                " - Takes the handle of a Signal, the value inside the Signal must be something that is iterable."
                </p>
                <p>
                <code class="one-liner">
                "template"
                </code>
                " - Place to put your html codes"
                </p>
                <p>
                <code class="one-liner">
                "key"
                </code>
                " - Takes a unique value for each iteration of elements"
                </p>

                <br/>
                <h3>"Example"</h3>
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
                file="src/main.rs"
                />
            
            <br/>
            <h3>"Indexed"</h3>
            <p>
            <code class="one-liner">
            "Indexed"
            </code>
            " is very similar to "<code class="one-liner">"Keyed"</code>". However, "<code class="one-liner">"Indexed"</code>" does not have a key field inside its props."
            </p>
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
           file="src/main.rs"
           />
           <h2>"Components"</h2>
           <p>"One can write components as functions starting with uppercase"</p>
           <CodePreview
            code="fn Todo(router: Router) -> Dom {}
html! {
  <Todo router={router} />
}
"
        file="src/main.rs"
           />
            </div>
        }
}
