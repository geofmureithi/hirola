use std::fmt::Display;

use hirola::prelude::*;

pub fn SeoTitle<T: Display + ?Sized>(title: &'static T) -> Dom {
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .set_title(&format!("{title}"));
    Dom::empty()
}
