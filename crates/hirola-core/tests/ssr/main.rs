use futures_signals::{signal::Mutable, signal_vec::MutableVec};
use hirola::prelude::*;

#[test]
fn hello_world() {
    let node = html! {
        <p>"Hello World!"</p>
    };

    assert_eq!(render_to_string(node), "<p>Hello World!</p>");
}

#[test]
fn reactive_text() {
    let count = Mutable::new(0);

    assert_eq!(
        render_to_string(
            html! {
                <p>{count.clone()}</p>
            }
        ),
        "<p>0</p>"
    );

    count.set(1);
    assert_eq!(
        render_to_string(
            html! {
                <p>{count}</p>
            }
        ),
        "<p>1</p>"
    );
}

#[test]
fn check_effects() {
    let count = MutableVec::new_with_values(vec![1, 2, 3]);

    let node = html! {
        <ul>
        {
            count.signal_vec().render_map(move |item| {
                html! {
                    <li>{item.to_string()}</li>
                }
            } )
        }
        </ul>
    };

    let view = render_to_string(node);
    assert_eq!("<ul><li>1</li><li>2</li><li>3</li><!----></ul>", view);
}
