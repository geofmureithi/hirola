use hirola::{prelude::*, signal::Mutable};

fn counter() -> Dom {
    let counter = Mutable::new(99);
    let decrement = counter.update_with(|c, _| {
        *c.lock_mut() -= 1;
    });
    let increment = counter.update_with(|c, _| {
        *c.lock_mut() += 1;
    });
    html! {
        <div>
            <button on:click=decrement>"-"</button>
            <span>{counter}</span>
            <span mixin:identity=&rhtml("<b>Test</b>") />
            <button on:click=increment>"+"</button>
        </div>
    }
}
fn main() {
    let dom = render(counter()).unwrap();
    std::mem::forget(dom);
}
