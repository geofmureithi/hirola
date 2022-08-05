mod model;
use std::future::Future;

use hirola::prelude::*;
use model::Users;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};

fn fetch_users(_app: &HirolaApp) -> Dom {
    let fetcher = async move {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let url = format!("https://jsonplaceholder.typicode.com/users");

        let request = Request::new_with_str_and_init(&url, &opts).unwrap();

        let window = web_sys::window().unwrap();

        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        let json = resp.json().unwrap();
        let json = JsFuture::from(json).await.unwrap();
        let users: Users = json.into_serde().unwrap();
        users
    };

    let users = use_async(fetcher);

    html! {
            <div class="grid h-screen place-items-center">
                {if users.get().is_none() {
                    html!{
                        <div class="h-10 w-32">"Loading..."</div>
                    }
                } else {
                    let users = &*users.get();
                    let users = users.clone().unwrap();

                    html! {
                        <div class="grid h-screen place-items-center">
                                {for user in users {
                                    html! {
                                        <div>
                                            {user.name.clone()}
                                        </div>
                                    }
                                }}
                        </div>
                    }
                }}
           </div>
    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", fetch_users);
}
