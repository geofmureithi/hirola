use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use hirola::prelude::*;

pub fn state_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <SeoTitle title={"State Management | Hirola"} />

            <h1>"State Management"</h1>
            <p>"Hirola is un-opinionated in state management. It should be pretty easy to roll out your own. To enable the global state management use the feature flag "<code class="one-liner">"global-state"</code></p>
            <h2>"Example"</h2>
            <CodePreview
             code=
r#"use hirola::prelude::*;
use web_sys::window;

#[derive(Clone)]
pub struct UserInfo {
    pub name: String,
    pub age: u8,
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    //App instance
    let mut app: HirolaApp = HirolaApp::new();


    //Routing
    let mut router: Router = Router::new();     
    router.add("/", home);                      
    router.add("/user/:id", user);
    app.extend(router);                         

    //Global State
    let demo_user = UserInfo{name: "Jeff".to_string(), age: 32};
    let global_state = Signal::new(demo_user);
    app.extend(global_state);

    //Starting App
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

fn home(app: &HirolApp) -> Dom {
    let user_info = app.data::<&UserInfo>().unwrap().clone();

    html! {
        <div>
            <p>"Welcome Home! "{user_info.name.clone()}", "{user_info.age.clone()}</p>
        </div>
    }
}
"#
            file="src/main.rs"
            />
        </div>
    }
}
