mod model;

use hirola::prelude::*;
use model::Users;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

async fn user_fetcher() -> Result<Users, JsValue> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;
    let window = web_sys::window().unwrap();
    let mut opts = RequestInit::new();
    opts.method("GET");
    let url = format!("https://jsonplaceholder.typicode.com/users");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = resp.json()?;
    let json = JsFuture::from(json).await?;
    let users: Users = json.into_serde().unwrap();
    Ok(users)
}

fn fetch_users() -> Dom {
    html! {
        <div class="grid h-screen place-items-center">
            <h1>"Users"</h1>
            {match user_fetcher().suspend().await {
                Ready(Ok(users)) => {
                    html! {
                        <ul>
                            {for user in users {
                                html! { <li>{user.name}</li> }
                            }}
                        </ul>
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

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let dom = render_to(fetch_users(), &body).unwrap();

    std::mem::forget(dom);
}
