use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn router_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
      <SeoTitle title={"Router | Hirola"} />

            <h1>"Router"</h1>
            <p>"Hirola is un-opinionated in route management. It should be pretty easy to roll out your own. To enable the inbuilt router use the feature flag "<code class="one-liner">"router"</code></p>
            <h2>"Getting started"</h2>
            <CodePreview
             code=
r#"let window = web_sys::window().unwrap();
let document = window.document().unwrap();
let body = document.body().unwrap();

let mut app = HirolaApp::new();

let mut router = Router::new();
router.add("/", home);
router.add("/todo/:id", todo_view);

app.mount(&body, |app| router.render(app));
"#
            file="src/main.rs"
            />
        </div>
    }
}
