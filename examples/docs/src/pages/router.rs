use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn router_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
      <SeoTitle title={"Router | Hirola"} />

            <h1>"Router"</h1>
            <p>"Hirola is un-opinionated in route management. It should be pretty easy to roll out your own. To enable the inbuilt router use the feature flag "<code class="one-liner">"router"</code></p>

            <br/>
            <h2>"Example"</h2>
            <CodePreview
             code=
r##"use hirola::prelude::*;
use websys::window;
fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let mut app: HirolaApp = HirolaApp::new();

    let mut router: Router = Router::new();     //create router instance
    router.add("/", home);                      //add all path
    router.add("/user/:id", user);
    app.extend(router);                         //extend our router onto our app instance

    app.mount(&body, |app: &HirolaApp| {
        let router = app.data::<Router>().unwrap().clone();
        let app = app.clone();

        html!{
            <div>
                {router.render(&app)}
            </div>
        }
    });
}

fn home(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <p>"Home Page"</p>
        </div>
    }
}

fn user(app: &HirolaApp) -> Dom {
    let router = app.data::<Router>().unwrap().clone();
    let route = router.params().get();
    let param: String = route.params.get("id").unwrap_or(&String::new()).clone();

    html! {
        <div>
            <p>"User: "{param.clone()}</p>
        </div>
    }
}
"##
            file="src/main.rs"
            />
        <p>
        "Here, we created a simple wasm-app that have 2 pages, "<code class="one-liner">"/home"</code>" and, "<code class="one-liner">"/user/:id"</code>
        ". We first start by creating our"<code class="one-liner">"Router"</code>" instance, modify it and extend it onto our "<code class="one-liner">"HirolaApp"</code>
        " instance. Finally, we starts our app by rendering the router at the root of the body"
        </p>

        <br/>
        <p>
        "Notice that "<code class="one-liner">"/user/:id"</code>" takes a parameter "<code class="one-liner">"id"</code>
        ". We can get the value of the parameter by using our Router instance that we extracted from HirolaApp."
        </p>
        </div>
    }
}
