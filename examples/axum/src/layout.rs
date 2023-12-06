use std::fmt::Display;

use hirola::{prelude::*, ssr::SsrNode};

const STYLE: &str = r#"
nav {
  border-bottom: 1px solid black;
}

.crumbs ol {
  list-style-type: none;
  padding-left: 0;
}

.crumb {
  display: inline-block;
}

.crumb a::after {
  display: inline-block;
  color: #000;
  content: '>';
  font-size: 80%;
  font-weight: bold;
  padding: 0 3px;
}
"#;
pub struct Seo {
    pub title: String,
    pub author: String,
    pub description: String, //..
}

#[component]
pub fn Layout<Nav, Main, Footer>(seo: Seo, nav: Nav, main: Main, footer: Footer) -> SsrNode
where
    Nav: Render<SsrNode>,
    Main: Render<SsrNode>,
    Footer: Render<SsrNode>,
{
    html! {
        <!DOCTYPE html>
        <html lang="en" dir="ltr">
            <head>
                <title>{&seo.title}</title>
                <meta charset="utf-8"/>
                <meta
                    name="viewport"
                    content="width=device-width, initial-scale=1, viewport-fit=cover"
                />
                <meta name="author" content=&seo.author/>
                <meta name="description" content=&seo.description/>
                <meta name="theme-color" content="#303030"/>
                <meta property="og:type" content="website"/>
                <meta property="og:title" content="Page Title less than 55 characters"/>
                <meta property="og:description" content=&seo.description/>
                <meta property="og:image" content="/some-image.png"/>
                <meta property="og:url" content="/this-page.html"/>
                <meta property="og:site_name" content="Your Site Name"/>
                <meta property="og:locale" content="en_US"/>
                <meta name="twitter:card" content="summary_large_image"/>
                <meta name="twitter:url" content="/this-page.html"/>
                <meta name="twitter:title" content="Page Title less than 55 characters"/>
                <meta name="twitter:image" content="/image.jpg"/>
                <link rel="icon" type="image/png" href="/favicon.png"/>
                <link
                    rel="apple-touch-icon"
                    type="image/png"
                    sizes="76x76"
                    href="/favicon.png?width=76"
                />
                <link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5"/>
                <link rel="canonical" href="/"/>
                <style>{STYLE}</style>
            </head>
            <body>
                <header>
                    <nav>{nav}</nav>
                </header>
                <main>{main}</main>
                <footer>{footer}</footer>
            </body>
        </html>
    }
}

fn skip_last<T>(mut iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let last = iter.next();
    iter.scan(last, |state, item| std::mem::replace(state, Some(item)))
}

#[component]
pub fn Nav<'a, T: Display>(items: &'a [T]) -> SsrNode {
    let last = items.last();

    let other_items = skip_last(items.iter());

    html! {
        <nav class="crumbs">
            <ol>
                {for item in other_items {
                    html! {
                        <li class="crumb">
                            <a href="#">{item.to_string()}</a>
                        </li>
                    }
                }}
                <li class="crumb">{last.unwrap().to_string()}</li>
            </ol>
        </nav>
    }
}
