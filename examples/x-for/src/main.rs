use hirola::{
    dom::*,
    prelude::*,
    signal_vec::{MutableVec, SignalVecExt},
};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, HtmlInputElement};

fn colors() -> Dom {
    let colors = MutableVec::new_with_values(
        vec!["Red", "Green", "Blue", "Violet"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect(),
    );
    let add_new = colors.callback_with(move |colors, e: Event| {
        e.prevent_default();
        let new_color: HtmlInputElement = window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("new_color")
            .unwrap()
            .dyn_into()
            .unwrap();
        let color = new_color.value();
        colors.lock_mut().push_cloned(color);
        new_color.set_value("");
        new_color.focus().unwrap();
    });

    html! {
        <>
            <h2>"Static"</h2>
            <ul>
                {for (_index, item) in (0..3).enumerate() {
                    html! { <li>{item.to_string()}</li> }
                }}
            </ul>
            <h2>"Reactive"</h2>
            <ul>
                {colors
                    .signal_vec_cloned()
                    .map_render(|item| {
                        html! { <li>{item}</li> }
                    })}
            </ul>
            <h2>"Reactive Filtered Starts with V"</h2>
            <ul>
                {colors
                    .signal_vec_cloned()
                    .filter(|color| color.starts_with('V'))
                    .map_render(|item| {
                        html! { <li>{item}</li> }
                    })}
            </ul>
            <form on:submit=add_new>
                <input id="new_color" type="text" required=""/>
                <button type="submit">"Add New Color"</button>
            </form>
        </>
    }
}

fn main() {
    mount(colors()).unwrap();
}
