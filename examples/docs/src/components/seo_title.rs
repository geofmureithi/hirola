use std::fmt::Display;

use hirola::prelude::*;

#[component]
pub fn SeoTitle<'a, T: Display + ?Sized>(title: &'a T) -> Dom {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(&format!("{title}"));
    Dom::new()
}
