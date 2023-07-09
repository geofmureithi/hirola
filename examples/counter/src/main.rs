use std::default;
use std::{marker::PhantomData, ops::Range};

use discard::DiscardOnDrop;
use futures::future::BoxFuture;
use futures::{Future, FutureExt, StreamExt, TryFutureExt};
use hirola::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::Response;
use web_sys::{window, Event, Request, RequestInit};

use hirola::prelude::html as view;

use serde::Deserialize;
use serde::Serialize;

pub type Users = Vec<User>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub email: String,
    pub address: Address,
    pub phone: String,
    pub website: String,
    pub company: Company,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub street: String,
    pub suite: String,
    pub city: String,
    pub zipcode: String,
    pub geo: Geo,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geo {
    pub lat: String,
    pub lng: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Company {
    pub name: String,
    pub catch_phrase: String,
    pub bs: String,
}

// #[derive(Debug, Clone)]
// struct App {
//     router: Router<Self>,
//     numbers: MutableVec<i32>,
// }

// impl App {
//     fn new(router: Router<Self>) -> Self {
//         Self {
//             router,
//             numbers: MutableVec::new_with_values(vec![1, 2, 3]),
//         }
//     }
//     fn mount(self, parent: &web_sys::Node) {
//         let display = NodeRef::new();
//         let node: NodeRef<DomType> = display.clone();
//         let router = self.router.clone();
//         let renderer = router
//             .render(self)
//             .signal_ref(move |dom| {
//                 let node = node.get::<DomNode>();
//                 let node = node.dyn_into::<web_sys::HtmlElement>().unwrap();
//                 node.replace_children_with_node_1(&dom.inner_element().inner_element());
//                 dom.on_mount();
//             })
//             .to_future();

//         render_to(
//             view! {
//                 <main use:renderer ref=display/>
//             },
//             parent,
//         );
//     }
// }

// struct If<S: Signal<bool>, F: Fn(bool) -> TemplateResult<DomType>> {
//     signal: S,
//     renderer: F,
// }

// impl<S: Signal<bool>, F: Fn(bool) -> TemplateResult<DomType>> Render for If<S, F> {
//     fn render(self) -> TemplateResult<G> {
//         let fragment = DomType::fragment();
//         self.signal.switch(|s| fragment );

//     }
// }

// #[component]
// fn If<F: Fn() -> Dom>(signal: Mutable<bool>, on: F, off: F) -> Dom {
//     let fragment = DomType::fragment();
//     let template = TemplateResult::new(fragment.clone());

//     let fut = signal
//         .signal_ref(|res| {
//             if *res {
//                 fragment.append_child(on());
//             } else {
//                 fragment.append_child(off());
//             }
//         })
//         .to_future();
//     template.effect(DiscardOnDrop::leak(spawn(fut)));
//     template
// }

#[derive(Debug, Default)]
pub enum SuspenseResult<Res> {
    #[default]
    Loading,
    Ready(Res),
}

trait Suspend {
    type Result;
    fn suspense(self) -> BoxedLocal<SuspenseResult<Self::Result>>;
}

impl<F, Res> Suspend for F
where
    F: FutureExt<Output = Res> + 'static,
{
    type Result = Res;
    fn suspense(self) -> BoxedLocal<SuspenseResult<Self::Result>> {
        Box::pin(self.map(|res| SuspenseResult::Ready(res)))
    }
}

async fn user_fetcher() -> Result<Users, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let window = web_sys::window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("https://jsonplaceholder.typicode.com/users?new1");
    let request = Request::new_with_str_and_init(&url, &opts)?;

    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = resp.json()?;
    let json = JsFuture::from(json).await?;
    let users: Users = json.into_serde().unwrap();
    Ok(users)
}

pub use SuspenseResult::*;

fn counter() -> ViewBuilder<DomNode> {
    // let router = &app.router;
    let values = MutableVec::new_with_values(vec![]);
    let add_one = values.update_with(|numbers, _e: Event| {
        let len: i32 = numbers.lock_ref().len().try_into().unwrap();
        numbers.lock_mut().push(len + 1);
    });

    let visible = Mutable::new(false);
    let toggle = visible.update_with(|visible, _e: Event| {
        visible.set(!visible.get());
    });

    fn remove_me(item: i32, values: MutableVec<i32>) {
        let mut values = values.lock_mut();
        let position = values.iter().position(|e| e == &item).unwrap();
        values.remove(position.try_into().unwrap());
    }
    let evens = values.clone();
    html! {
        <div>
            <h2>"Static"</h2>
            <ul>
                {for (index, item) in (0..3).enumerate() {
                    html! {
                        <li on:click=move |_| log::debug!("Clicked {index}")>{item.to_string()}</li>
                    }
                }}
            </ul>
            <h2>"All"</h2>
            <ul>
                {values
                    .signal_vec()
                    .render_map(|item| {
                        html! {
                            <li on:click=move |_| log::info!("Clicked")>{item.to_string()}</li>
                        }
                    })}
            </ul>
            <ul>
                {for item in values.signal_vec() {
                    let values = values.clone();
                    html! {
                        <li on:click=move |_| remove_me(
                            item,
                            values.clone(),
                        )>{item.to_string()}</li>
                    }
                }}
            </ul>
            <h2>"Evens"</h2>
            <div>
                {match user_fetcher().suspense().await {
                    Ready(Ok(users)) => {
                        let evens = evens.clone();
                        html! {
                            <div>
                                "Some data here"
                                {match user_fetcher().suspense().await {
                                    Ready(Ok(users)) => {
                                        html! {
                                            <div>
                                                {users.len().to_string()}
                                                {evens
                                                    .clone()
                                                    .signal_vec()
                                                    .filter(|c| *c % 2 == 0)
                                                    .render_map(|item| {
                                                        html! {
                                                            <li on:click=move |_| {
                                                                log::info!("Clicked {item}")
                                                            }>{item.to_string()}</li>
                                                        }
                                                    })}
                                            </div>
                                        }
                                    }
                                    Ready(Err(err)) => {
                                        html! { <div>"An error occurred"</div> }
                                    }
                                    _ => {
                                        html! { <div>"Loading..."</div> }
                                    }
                                }}
                            </div>
                        }
                    }
                    Ready(Err(err)) => html! { <div>"An error occurred"</div> },
                    _ => html! { <div>"Loading..."</div> },
                }}
            </div>
            <button on:click=toggle>"Change Visibility"</button>
            <div>
                {if visible.signal() as _ {
                    html! { <p>"Its true"</p> }
                } else {
                    html! { <p>"Its false"</p> }
                }}
            </div>
            "Text"
            {ideal()}
            <button on:click=add_one>"Add Next"</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    counter().mount(&DomNode { node: body.into() }).unwrap();
}

fn ideal() -> ViewBuilder<DomNode> {
    let counter = Mutable::new(0);
    let increment = counter.update_with(|v, _| {
        v.set(v.get() + 1);
    });
    html! {
        <div>
            <ul>
                {for item in (0..3).enumerate() {
                    html! { <li>"Her name is Kitty White."</li> }
                }}
            </ul>
            <p>
                Welcome to Hirola
            </p>
            <button on:click=increment>
                Increment
            </button>
            <p>{counter}</p>
        </div>
    }
}
