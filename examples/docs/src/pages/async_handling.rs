use crate::components::code_preview::CodePreview;
use crate::components::seo_title::SeoTitle;
use crate::App;
use hirola::prelude::*;

pub fn async_page(_app: &App) -> Dom {
    html! {
        <div>
            <SeoTitle title="Async handling | Hirola"/>
            <h1>"Async handling"</h1>
            <p>
                "Hirola allows some async handling via " <code class="one-liner">"async"</code>
                " feature."
            </p>
            <h2>"Example"</h2>
            <CodePreview
                code= r###"
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
                
                fn fetch_users(_app: &App<S, G>) -> Dom {
                    let users: AsyncResult<Users> = use_async(fetcher());
                    html! {
                        <div class="grid h-screen place-items-center">
                            {if users.get().is_none() {
                                html!{
                                    <div class="h-10 w-32">"Loading..."</div>
                                }
                            }  else {
                    ......"###

                file="main.rs"
            />
        </div>
    }
}
