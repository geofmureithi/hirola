use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use crate::App;
use hirola::prelude::*;

pub fn ssr_page(_app: &App) -> Dom {
    html! {
        <div>
            <SeoTitle title="Server Side Rendering | Hirola"/>
            <h1>"Server Side Rendering"</h1>
            <p>
                "Hirola supports basic server side rendering with the feature "
                <code class="one-liner">"ssr"</code> "."
            </p>
            <h2>"Example"</h2>
            <CodePreview
                code="fn main(){
                    let app = HirolaApp::new();
                    let res = app.render_to_string(counter);
                    assert_eq!("<div><button>Increment</button><span>0</span></div>", &res);
                }"
                file="main.rs"
            />
        </div>
    }
}
