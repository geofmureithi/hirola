use hirola::prelude::*;

fn counter(app: &HirolaApp) -> Dom {
    let count = Signal::new(0);
    html! {
        <div>
            <button on:click={count.mut_callback(|c, _| c + 1)}>"Increment"</button>
            <span>{count.get()}</span>
        </div>
    }
}
fn main() {
    let app = HirolaApp::new();
    app.mount("body", counter);
}
