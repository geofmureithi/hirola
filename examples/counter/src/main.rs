use gloo_timers::future::TimeoutFuture;
use hirola::dom::Dom;
use hirola::prelude::*;
use hirola::dom::effects::prelude::*;

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    // let decrement = count.callback(|s| *s.lock_mut() -= 1);
    // let increment = count.callback(|s| *s.lock_mut() += 1);
    // let counter = count.clone();
    let second_counter = async move {
        web_sys::window().unwrap().alert_with_message("Incr");
        // loop {
            
        //     TimeoutFuture::new(1_000).await;
        //     *counter.lock_mut() += 1;
        // }
    };

    html! {
        <>
            // <button on:click=decrement>"-"</button>
            <span use:future=second_counter>{count}</span>
            // <button on:click=increment>"+"</button>
        </>
    }
}

fn main() {
    hirola::dom::mount(counter()).unwrap();
}
