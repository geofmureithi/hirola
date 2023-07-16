use hirola::{prelude::*, signal::Mutable};

fn counter() -> DomBuilder {
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
            <button on:click=increment>"+"</button>
        </div>
    }
}
fn main() {
    let view = render(counter()).unwrap();
    std::mem::forget(view);
}
