use hirola::prelude::*;
use hirola::signal::Mutable;
use hirola_dom::{Dom, Event};
use std::{fmt::Display, future::ready};
use web_sys::window;

#[component]
pub fn Menu<R: Render<Dom>>(title: &'static str, children: R) -> Dom {
    html! {
        <menu>{children}</menu>
    }
}

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    let alert = Box::new(|e| window().unwrap().alert_with_message("Test").unwrap());
    let my_future = ready(());
    html! {
        <>
            <button on:click=decrement>"-"</button>
            <span use:my_future on:click={alert}>{count}</span>
            <button on:click=increment>"+"</button>
            // <Menu title="Test" children={"Child"}/>
        </>
    }
}

fn main() {
    let root = hirola_dom::render(counter()).unwrap();
    std::mem::forget(root);
}
