use hirola::{prelude::*, ssr::SsrNode};

#[component]
pub fn HirolaLogo() -> SsrNode {
    html! {
        <h1 class="text-5xl text-orange-600" style="font-family: 'Grape Nuts', cursive;">
            "Hirola"
        </h1>
    }
}
