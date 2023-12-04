use hirola::prelude::*;
use hirola::signal::Mutable;
use hirola::signal_vec::SignalVecExt;
use hirola_dom::{mixins::raw_text, node_ref::NodeRef, Dom, Event};
use std::{fmt::Display, future::ready};
use web_sys::window;

#[component]
pub fn Menu(title: &'static str, children: Box<dyn Fn(&str) -> Dom>) -> Dom {
    html! {
        <>
            <menu>{children(title)}</menu>
        </>
    }
}

fn counter() -> Dom {
    let count = Mutable::new(0i32);
    let decrement = count.callback(|s| *s.lock_mut() -= 1);
    let increment = count.callback(|s| *s.lock_mut() += 1);
    let alert = Box::new(|_| window().unwrap().alert_with_message("Test").unwrap());
    let node_ref = NodeRef::new();
    let my_future = ready(());
    let colors = MutableVec::new_with_values(vec!["Red", "Green", "Blue", "Violet"]);
    let add_new = colors.callback_with(move |colors, _e: Event| {
        colors.lock_mut().push("Violet-Dark");
    });
    html! {
        <>
            <span>"Text"</span>
            <button ref={node_ref} on:click=decrement>"-"</button>
            <span on:click=alert>{count.clone()}</span>
            <button on:click=increment>"+"</button>
            <Menu title="Test" children=Box::new(|text| html! { <>{text}</> })/>
            <span bind:attribute=count/>
            <span mixin:identity=&raw_text("Boook!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!") />
            <ul>
                {for (_index, item) in (0..3).enumerate() {
                    html! { <li>{item.to_string()}</li> }
                }}
            </ul>
            <h2>"Reactive"</h2>
            <ul>
                {colors
                    .signal_vec()
                    .render_map(|item| {
                        html! { <li>{item}</li> }
                    })
                }
            </ul>
            <button on:click=add_new>"Add new color"</button>
            <h2>"Reactive Filtered Starts with V"</h2>
            <ul>
                {colors
                    .signal_vec()
                    .filter(|color| color.starts_with("V"))
                    .render_map(|item| {
                        html! { <li>{item}</li> }
                    })
                }
            </ul>
        </>
    }
}

fn main() {
    let root = hirola_dom::render(counter()).unwrap();
    std::mem::forget(root);
}
