use crate::{components::seo_title::SeoTitle, App};
use hirola::prelude::*;

pub fn extending_page(_app: &App) -> Dom {
    html! {
      <div>
      <SeoTitle title={"Extending | Hirola"} />

        <h1>"Extending"</h1>
        <p>"Hirola supports extending via mixins and can be enabled by feature"<code class="one-liner">"mixins"</code>"."</p>
        <p>"Some rules about mixins"</p>
        <ul>
            <li>"Mixins are executed sequentially"</li>
            <li>"Mixins should be the last attributes on a dom attrubutes"</li>
            <li>"A good mixin receives a signal and interacts with the bound node"</li>
        </ul>
        <blockquote>
            <p>"Hirola recommends you package your plugin and publish it to crates.io with the hirola- prefix. The hirola-form plugin is a good example."
                <a href="https://docs.rs/hirola-form/latest/hirola_form/">"→ See example on docs.rs"</a>
            </p>
        </blockquote>
      </div>
    }
}
