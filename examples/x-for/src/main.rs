use hirola::prelude::*;

fn colors(app: &HirolaApp) -> Dom {
    let colors = Signal::new(vec!["Red", "Green", "Blue"]);

    html! {
        <ul>
            <template x-for="color in colors">
                <li>{color}</li>
            </template>
        </ul>
    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", colors);
}
