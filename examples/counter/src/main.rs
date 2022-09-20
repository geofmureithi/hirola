use hirola::prelude::*;

fn counter(_app: &HirolaApp) -> Dom {
    let count = Signal::new(0);
    html! {
        <div>
            <button on:click=count.mut_callback(|c, _| c + 1)>"Increment"</button>
            <span>{count.get()}</span>
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
