use std::fmt::Display;

use hirola::prelude::*;

#[component]
pub fn SeoTitle<T: Display + 'static>(title: T) -> Dom {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(&format!("{title}"));
    Dom::new()
}
