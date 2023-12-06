use gloo_timers::future::TimeoutFuture;
use hirola::dom::Dom;
use hirola::prelude::*;

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    let counter = count.clone();
    let second_counter = async move {
        loop {
            TimeoutFuture::new(1_000).await;
            *counter.lock_mut() += 1;
        }
    };

    html! {
        <>
            <button on:click=decrement>"-"</button>
            <span use:second_counter>{count}</span>
            <button on:click=increment>"+"</button>
        </>
    }
}

fn main() {
    let root = hirola::dom::render(counter()).unwrap();
    std::mem::forget(root);
}
