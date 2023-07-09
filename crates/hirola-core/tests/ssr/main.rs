use hirola::prelude::*;

#[test]
fn hello_world() {
    let node = html! {
        <p>"Hello World!"</p>
    };

    assert_eq!(render_to_string(|| node), "<p>Hello World!</p>");
}

#[test]
fn reactive_text() {
    let count = Mutable::new(0);

    let node = cloned!((count) => html! {
        <p>{ (count.get()) }</p>
    });

    assert_eq!(render_to_string(cloned!((node) => || node)), "<p>0</p>");

    count.set(1);
    assert_eq!(render_to_string(|| node), "<p>1</p>");
}
