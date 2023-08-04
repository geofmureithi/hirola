---
title: Async handling with hirola.
date: "2023-01-21"
tags: ["rust", "hirola", "basics", "starter"]
summary: We are going to learn how to handle event handling using hirola
draft: false
---

# Async handling

## Using Suspense

Hirola allows some async handling via `wasm-bindgen-futures`.
Consider this example:

```rust
async fn fetcher() -> Result<Users, JsValue> {
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
```

You can mount this future on the dom:

```rust
html!{
    {match fetcher().suspend().await {
            Loading => html! { <div>"Loading..."</div> },
            Ready(Ok(users)) => {
                html! {
                    <ul>
                        {for user in users {
                            html! { <li>{user.name}</li> }
                        }}
                    </ul>
                }
            },
            Ready(Err(err)) => html! { <div>"An error occurred: " {err.to_string()}</div> }
        }
    }
}
```

## Using side effects

Side effects are futures that may never complete.
Any future can be used as a side-effect, and is dropped when it is complete or the dom item attached to it is dropped.

```rust
let effect = async {
    loop {
        //......
    }
};

html! {
    <div use:effect />
}

```
