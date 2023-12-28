mod model;
use anyhow::bail;
use hirola::dom::*;
use hirola::prelude::*;
use hirola::prelude::Suspend;
use model::Users;
use reqwasm::http::Request;

async fn user_fetcher() -> anyhow::Result<Users> {
    let request = Request::get("https://jsonplaceholder.typicode.com/users");
    let response = request.send().await?;
    if response.status() == 200 {
        Ok(response.json().await?)
    } else {
        bail!(
            "Failed with status {}, {}",
            response.status(),
            response.text().await?
        )
    }
}

fn fetch_users() -> Dom {
    html! {
        <div class="grid h-screen place-items-center">
            <h1>"Users"</h1>
            {match user_fetcher().suspend().await {
                Loading => {
                    html! { <div>"Loading..."</div> }
                }
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
                    html! { <div>"An error occurred: " {err.to_string()}</div> }
                }
            }}
        </div>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let dom = hirola::dom::render_to(fetch_users(), &body).unwrap();

    std::mem::forget(dom);
}
