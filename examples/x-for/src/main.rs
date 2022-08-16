use hirola::prelude::*;

fn Color(text: Signal<String>) -> Dom {
    html! {
        <input
            class="block"
            mixins=vec![bind_input(&text)]
        />
    }
}

fn colors(_app: &HirolaApp) -> Dom {
    let colors = Signal::new(
        vec!["Red", "Green", "Blue"]
            .iter()
            .map(|c| Signal::new(c.to_string()))
            .collect::<Vec<Signal<String>>>(),
    );
    let add_new = colors.callback(move |colors, _e: Event| {
        colors.push(Signal::new("New Color".to_string()));
    });

    html! {
        <div>
            <Indexed
                props={
                    IndexedProps {
                        iterable: colors.clone().handle(),
                        template: |item| {
                            html! {
                                <Color text=item />
                            }
                        }
                    }
                }
            />
            <Keyed
                props={
                    KeyedProps {
                        iterable: colors.clone().handle(),
                        template: |item| {
                            html! {
                                <p>{item.get()}</p>
                            }
                        },
                        key: |item| item.get().clone()
                    }
                }
            />
            <button on:click=add_new>"Add New"</button>
        </div>

    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", colors);
}
