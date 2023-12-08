//! Run with
//!
//! ```not_rust
//! cargo run -p axum-example
//! ```

mod layout;

use crate::layout::*;

use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use hirola::{
    prelude::*,
    ssr::{render_to_string, SsrNode},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

async fn home() -> impl IntoResponse {
    let posting = Seo {
        author: "Jude Bellingham".to_owned(),
        description: "This BMX bike is a solid step into the pro world. It looks as legit as it rides and is built to polish your skills.".to_owned(),
        title: "Jump Bike 3000".to_owned(),
    };

    let template = html! {
        <>
            <Layout
                seo=posting
                nav={
                    html! { <Nav items=&["Bikes", "BMX", "Jump Bike 3000"]/> }
                }
                main={
                    html! {
                        <>
                            <h2>{&posting.title}</h2>
                            <p>{&posting.description}</p>
                        </>
                    }
                }
                footer=SsrNode::fragment()
            />
        </>
    };
    SsrTemplate(template)
}

struct SsrTemplate(SsrNode);

impl IntoResponse for SsrTemplate {
    fn into_response(self) -> Response {
        match render_to_string(self.0) {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err:?}"),
            )
                .into_response(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_example=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new().route("/", get(home));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
