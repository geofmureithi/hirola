use hirola::prelude::{*, mixins::text};
use web_sys::window;

fn counter(_app: &HirolaApp) -> Dom {
    let count = Signal::new(0);
    create_effect(count.clone(), |val| {
        // window().unwrap().alert_with_message(&format!("Count is at {val}")).unwrap();
    });
    
    html! {
        <div>
            <button style="margin:5px"  on:click=count.mut_callback(|c, _| c - 1)>"-"</button>
            <span mixin::text=&text(&count)/>
            <button style="margin:5px" on:click=count.mut_callback(|c, _| c + 1)>"+"</button>
        </div>

    }
}
fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let app = HirolaApp::new();
    app.mount(&body, counter);
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    #[wasm_bindgen_test]
    fn counter_renders() {
        let app = HirolaApp::new();
        let res = app.render_to_string(counter);
        assert_eq!("<div><button>Increment</button><span>0</span></div>", &res);
    }
}
