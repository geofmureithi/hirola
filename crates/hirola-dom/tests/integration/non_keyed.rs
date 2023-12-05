use hirola_core::prelude::{signal::Mutable, signal_vec::MutableVec};
use hirola_dom::dom_test_utils::{next_tick, next_tick_with};

use super::*;

#[wasm_bindgen_test]
fn append() {
    let count = MutableVec::new_with_values(vec![1, 2]);

    let node = html! {
        <ul>
            {
                count.signal_vec().render_map(|item| {
                    html! {
                        <li>{ item.to_string() }</li>

                    }
                } )
            }
        </ul>
    };

    let _ = render_to(node, &test_div());

    let p = document().query_selector("ul").unwrap().unwrap();

    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "12");
    });

    count.lock_mut().push(3);
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "123");
    });
    let new_value = count.lock_ref()[1..].to_vec();
    count.lock_mut().replace(new_value);
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "23");
    });
}

#[wasm_bindgen_test]
fn swap_rows() {
    let count = MutableVec::new_with_values(vec![1, 2, 3]);

    let node = html! {
        <ul>
        {
            count.signal_vec().render_map(|item| {
                html! {
                    <li>{ item.to_string() }</li>

                }
            } )
        }
        </ul>
    };

    let _ = render_to(node, &test_div());

    let p = document().query_selector("ul").unwrap().unwrap();
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "123");
    });

    count.lock_mut().swap(0, 2);
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "321");
    });

    count.lock_mut().swap(0, 2);
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "123");
    });
}

#[wasm_bindgen_test]
fn delete_row() {
    let count = MutableVec::new_with_values(vec![1, 2, 3]);

    let node = html! {
        <ul>
        {
            count.signal_vec().render_map(|item| {
                html! {
                    <li>{ item.to_string() }</li>

                }
            } )
        }
        </ul>
    };

    let _ = render_to(node, &test_div());

    let p = document().query_selector("ul").unwrap().unwrap();
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "123");
    });

    count.lock_mut().remove(1);
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "13");
    });
}

#[wasm_bindgen_test]
fn clear() {
    let count = MutableVec::new();

    let node = html! {
        <ul>
        {
            count.signal_vec().render_map(|item: i32| {
                html! {
                    <li>{ item.to_string() }</li>
                }
            } )
        }
        </ul>
    };

    let _ = render_to(node, &test_div()).unwrap();

    let p = document().query_selector("ul").unwrap().unwrap();
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "");
    });
    count.lock_mut().replace(vec![1, 2, 3]);
    next_tick_with(&p, |p| {
        assert_eq!(p.inner_html(), "123");
    });
    count.lock_mut().replace(Vec::new());
    next_tick_with(&p, |p| {
        assert_eq!(p.text_content().unwrap(), "");
    });
}

#[wasm_bindgen_test]
fn insert_front() {
    let count = MutableVec::new_with_values(vec![1, 2, 3]);

    let node = html! {
        <ul>
        {
            count.signal_vec().render_map(|item| {
                html! {
                    <li>{ item.to_string() }</li>
                }
            } )
        }
        </ul>
    };

    let _ = render_to(node, &test_div());
    next_tick(|| {
        let p = document().query_selector("ul").unwrap().unwrap();
        assert_eq!(p.text_content().unwrap(), "123");
    });
    count.lock_mut().insert(0, 4);
    next_tick(|| {
        let p = document().query_selector("ul").unwrap().unwrap();
        assert_eq!(p.text_content().unwrap(), "4123");
    });
}

#[wasm_bindgen_test]
fn nested_reactivity() {
    let count =
        MutableVec::new_with_values(vec![1u32, 2, 3].into_iter().map(Mutable::new).collect());

    let node = html! {
        <ul>
        {
            count.signal_vec_cloned().render_map(|item| {
                html! {
                    <li>{ item }</li>

                }
            } )
        }
        </ul>
    };

    let _ = render_to(node, &test_div());

    next_tick(|| {
        let p = document().query_selector("ul").unwrap().unwrap();
        assert_eq!(p.text_content().unwrap(), "123");
    });

    count.lock_ref()[0].set(4);
    next_tick(|| {
        let p = document().query_selector("ul").unwrap().unwrap();
        assert_eq!(p.text_content().unwrap(), "423");
    });

    count.lock_mut().push_cloned(Mutable::new(5));
    next_tick(|| {
        let p = document().query_selector("ul").unwrap().unwrap();
        assert_eq!(p.text_content().unwrap(), "4235");
    });
}
