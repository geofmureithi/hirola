use hirola::prelude::*;
use wasm_bindgen::prelude::Closure;
use crate::components::seo_title::SeoTitle;

pub fn browser_instance_page(_app: &HirolaApp) -> Dom {
    html! {
        <div>
            <SeoTitle title={"Browser Instance | Hirola"} />
            <h1>"Browser Instance"</h1>
            <p>
            "Hirola work hand-on-hand with "
                <code class="one-liner">
                "web_sys"
                </code>
            " to get all the browser instances"
            </p>
        </div>
    }
}