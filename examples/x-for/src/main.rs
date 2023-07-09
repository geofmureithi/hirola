use std::fmt::Display;

use hirola::prelude::{mixins::model::input, *};
use web_sys::Event;

#[component]
fn Color(text: Mutable<String>) -> Dom {
    html! {
        <div
            class="block"
            mixin:model=&input(&text)
        />
    }
}

#[component]
fn UserFn<T: Display>(name: T) {
    let text = format!("Hello, {}", name);
    html! {
        <div>{text.clone()}</div>
    }
}

struct User {
    name: String,
}

impl hirola::prelude::Render<DomType> for User {
    fn render(&self) -> Dom {
        let text = format!("Hello, {}", self.name);
        html! {
            <div>{text.clone()}</div>
        }
    }
}

#[component]
fn Page<'a, Children: Render<DomType>>(title: &'a str, children: Children) {
    let text = format!("Hello, {}", title);
    let children = children.render();
    html! {
        <>
            <div>
                {children.clone()}
            </div>
            <p>{text.clone()}</p>
        </>
    }
}

fn colors(_app: &HirolaApp) -> Dom {
    let colors = Mutable::new(
        vec!["Red", "Green", "Blue"]
            .iter()
            .map(|c| Mutable::new(c.to_string()))
            .collect::<Vec<Mutable<String>>>(),
    );
    let add_new = colors.update(move |colors, _e: Event| {
        colors.push(Mutable::new("New Color".to_string()));
    });

    html! {
        <>
            <Page title=&"Test Page">
                <User name=String::from("Geoff2") />
                <UserFn name=String::from("Mureithi2") />
                <button on:click=add_new>"Add New"</button>
            </Page>
        </>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let app = HirolaApp::new();
    app.mount(&body, colors);
}
