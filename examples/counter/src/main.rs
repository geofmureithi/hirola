use std::fmt::Display;
use hirola::prelude::*;
use hirola::signal::Mutable;

#[component]
pub fn Menu<R: Render>(title: &'static str, children: R) -> Dom {
    Dom::new()
}


fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    html! {
        <>
            <button on:click=decrement>"-"</button>
            <span>{count}</span>
            <button on:click=increment>"+"</button>
            <Menu title="Test" children={"Child"}/>
        </>
    }
}

fn main() {
    let root = render(counter()).unwrap();
    std::mem::forget(root);
}
