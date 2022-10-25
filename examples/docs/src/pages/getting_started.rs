use hirola::prelude::*;

use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;

const INDEX: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
  </head>
</html>
"#;

pub fn getting_started_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
          <SeoTitle title={"Getting Started | Hirola"} />

            <h1>"Prerequisites"</h1>
            <p>
              "Before getting started with"
              <code class="one-liner">"hirola"</code>
              " we are going to assume that you have the following tools installed:"
            </p>
            <ul class="ml-4">
              <li>"Rust"</li>
              <li>"Cargo"</li>
              <li>"Trunk"</li>
            </ul>
            <h1>"Getting Started"</h1>
            <p>"We are going to create a simple counter program."</p>
            <code class="block one-liner my-1 py-1">
              " > cargo new counter"
            </code>
            
            <br/>
            <p>"With a new project, we need to create an index file which is the entry point and required by trunk"</p>
            <code class="block one-liner my-1 py-1">
              " > cd counter"
            </code>

            <br/>
            <p>"Create an "<b>"index.html"</b>" in the root of counter. Add the contents below"</p>
            <CodePreview code={INDEX} file="index.html" />

            <br/>
            <p>"Lets add some code to "<b>"src/main.rs" </b></p>
            <CodePreview code={include_str!("../../../counter/src/main.rs")} file="src/main.rs" />
            <p>
            "Here, what we did is pretty simple, "
            </p>

            <br/>
            <p>
            "In "
              <code class="one-liner">
              "fn main()"
              </code>
            ", we get the instance of the body with the help of "
              <code class="one-liner">
              "web_sys"
              </code>
            " library and mount our HirolaApp onto the body. The "
              <code class="one-liner">
              "HirolaApp"
              </code>
            " is just a struct that holds the context of your app. We can use this later for our routing and global state management.
            As you can see, "
              <code class="one-liner">
              "app.mount()"
              </code>
            " has 2 parameters, location to mount, and the thing to mount. In this case, the thing to mount is just a function that return a normal DOM."
            </p>

            <br/>
            <p>
            "In "
              <code class="one-liner">
              "fn counter(_app: &HirolaApp)"
              </code>
            "we take a reference of"
              <code class="one-liner">
              "HirolaApp"
              </code>
            " and returns a DOM. The parameter will help us get access to our router and global state, but for now, for the sake of simplicity, we just gonna ignore it.
              First thing we do in this function is create a "
              <code class="one-liner">
              "Signal"
              </code>
            ". Basically, what it does is it helps us with the reactivity when handling reactive data. Next, we create our DOM with the help of "
              <code class="one-liner">
              "html! {}"
              </code>
            " macro. You can put anything that is html-valid and some of Hirola additional syntaxes in it to make it easy for developers when writing wasm front-end."
            </p>

            <br/>
            <p>
            "Lastly, we set a test module to make sure we are good to go. And that's it!"
            </p>


            <br/>
            <p>"Now lets run our project"</p>
            <code class="block one-liner my-1 py-1">
              " > trunk serve"
            </code>

            <br/>
            <p>"You should be able to get counter running."</p>
            <p class="text-xs"><span>"Try it out"</span></p>
              <div class="demo">
              {
                  let count = Signal::new(0);
                  html! {
                      <div>
                        <button on:click=count.mut_callback(|c, _| c + 1)>"Increment"</button>
                        <span class="ml-1">{count.get()}</span>
                      </div>
                    }
                }
              </div>

              <br/>
              <p>"We can also test our project using wasm-pack"</p>
              <code class="block one-liner my-1 py-1">
                " > wasm-pack test --node"
              </code>

              <br/>
              <h1>"Third Party Template"</h1>
              <p>"Alternatively, you can start your hirola project using one of the third party template created by the community."</p>
              <p>"To import it, you first need to install "<code class="one-liner">"cargo generate"</code>" extension."</p>
              <p>"simply run this command to install"</p>
              <code class="block one-liner">" > cargo install cargo-generate"</code>
              <br/>
              <p>"After the installation compelete, you can use the extension and import the template by running this command"</p>
              <code class="block one-liner">" > cargo generate Najidnadri/hirola_template "</code>
        </div>
    }
}
