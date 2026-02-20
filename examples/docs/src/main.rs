mod components;
use std::{fs::File, path::PathBuf};

use components::logo::HirolaLogo;
use comrak::{markdown_to_html_with_plugins, ComrakPlugins};
use hirola::{
    prelude::*,
    ssr::{render_to_string, SsrNode},
};

use crate::components::side_bar::SideBar;
use serde::Deserialize;

use comrak::plugins::syntect::SyntectAdapter;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Seo {
    title: String,
    date: Option<String>,
    tags: Vec<String>,
    summary: String,
    draft: bool,
}

fn with_layout(seo: Seo) -> SsrNode {
    html! {
        <html>
        <head>
          <title>{seo.title.clone()} " | Hirola documentation"</title>
          <meta charset="utf-8"/>
          <meta name="viewport" content="width=device-width, initial-scale=1, viewport-fit=cover"/>
          <meta name="author" content="geofmureithi"/>
          <meta name="description" content={&seo.summary}/>
          <meta name="theme-color" content="#303030"/>
          <meta property="og:type" content="website"/>
          <meta property="og:title" content={&seo.title}/>
          <meta property="og:description" content={&seo.summary}/>
          <meta property="og:image" content="/public/some-image.png"/>
          <meta property="og:url" content="/public/this-page.html"/>
          <meta property="og:site_name" content="Hirola Docs"/>
          <meta property="og:locale" content="en_US"/>
          <meta name="twitter:card" content="summary"/>
          <meta name="twitter:title" content={&seo.title}/>
          <meta name="twitter:image" content="/public/image.jpg"/>
          <link rel="icon" type="image/png" href="/public/favicon.png" />
          <link rel="apple-touch-icon" type="image/png" sizes="76x76" href="/favicon.png?width=76" />
          <link rel="mask-icon" href="/public/safari-pinned-tab.svg" color="#5bbad5" />
          <link rel="canonical" href="/" />
          <script src="https://cdn.tailwindcss.com"></script>
            <style>
            {r#"
                @import url("https://fonts.googleapis.com/css2?family=Grape+Nuts&display=swap");
            "#}
            </style>
        </head>
        <body>
        <div>
            <header class="bg-white md:fixed md:left-0 md:right-0 md:top-0 md:z-30 md:h-[5rem]">
                <div class="flex items-center justify-between pt-3">
                    <div class="py-0 pl-6 text-2xl font-semibold text-gray-800 hover:text-gray-900 md:w-64">
                        <a href="/" class="flex items-center">
                            <div class="w-[200px] md:w-[290px]">
                                <HirolaLogo/>
                            </div>
                        </a>
                    </div>
                    <div class="hidden items-center justify-end space-x-6 py-4 pr-6 text-gray-800 md:flex">
                        <a
                            href="https://github.com/geofmureithi/hirola"
                            class="block hover:text-gray-600"
                        >
                            "GitHub"
                        </a>
                    </div>
                    <div class="flex items-center pr-6 md:hidden">
                        <input type="checkbox" id="mobile-nav-toggle" class="hidden peer" />
                        <label for="mobile-nav-toggle" class="text-gray-600 cursor-pointer focus:outline-none">
                            <span class="sr-only">"Show navigation"</span>
                            <svg
                                class="h-6 w-6 fill-current"
                                xmlns="http://www.w3.org/2000/svg"
                                viewBox="0 0 20 20"
                            >
                                <path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"></path>
                            </svg>
                        </label>
                        <div
                            class="fixed top-0 bottom-0 right-0 z-[199] hidden w-1/2 overflow-y-auto bg-gray-100 p-6 shadow-xl peer-checked:block md:top-[4rem]"
                        >
                            <div class="flex justify-end">
                                <label for="mobile-nav-toggle" class="text-gray-600 cursor-pointer focus:outline-none">
                                    <span class="sr-only">"Close navigation"</span>
                                    <svg
                                        class="h-6 w-6"
                                        stroke="currentColor"
                                        fill="none"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M6 18L18 6M6 6l12 12"
                                        ></path>
                                    </svg>
                                </label>
                            </div>
                            <SideBar/>
                        </div>
                    </div>
                </div>
            </header>
            <style>
                "[hover-scrollbar] { overflow-x: 'visible'; overflow-y: hidden; } [hover-scrollbar]:hover { overflow-y: auto }"
            </style>
            <aside
                class="fixed left-0 bottom-0 hidden w-48 px-8 pb-6 pt-8 md:top-[4rem] md:block lg:w-64"
                hover-scrollbar=true
            >
                <SideBar/>
            </aside>
            <main class="pt-32 pl-0 pr-0 md:pl-48 lg:pl-64 xl:pr-64">
                <style>
                    {r##"
            .markdown>*+* { margin-top: 1.5rem; }
            .markdown p { line-height: 1.75; }
            .markdown h1, .markdown h2, .markdown h3 { color: #2d3342; scroll-margin-top: 4rem; font-weight: 800; letter-spacing: -.025em; }
            .markdown h1, .markdown h2 { border-bottom-width: 1px; border-color: #edf2f7; }
            .markdown h1 { font-size: 2.25rem; }
            .markdown h2 { font-size: 1.5rem; }
            .markdown h3 { font-size: 1.25rem; }
            .markdown ul { list-style-type: disc; }
            .markdown a { color: hsl(192deg 51% 46%); font-weight: 600; }
            .markdown h1 a, .markdown h2 a, .markdown h3 a { position: relative; color: #2d3342; font-weight: 800; }
            .markdown h1 a::before, .markdown h2 a::before, .markdown h3 a::before {
                position: absolute;
                left: -1.35rem;
                content: "# ";
                color: rgba(175, 187, 199);
                font-weight: 600;
            }
            .markdown h1 a:hover::before, .markdown h2 a:hover::before, .markdown h3 a:hover::before {
                color: #7e8a9e;
            }
            .fill-current {
              fill: "black";
            }
            .markdown code.one-liner { white-space: nowrap; padding-top: .1rem; padding-bottom: .1rem; padding-left: .25rem; padding-right: .25rem; font-size: .875rem; font-weight: 600; border-radius: .25rem; background-color: #edf2f7; }
            .markdown pre code {
              font-size: 12px;
              margin-top: 6px;
              margin-bottom: 6px;
            }
            .markdown blockquote { font-size: .95em; color: #2d3748; padding: 1rem; border-left-width: 4px; border-color: rgba(119, 193, 210); background: #f6f7f9; }
            .markdown table tbody td { border-color: #d8dee9; border-width: 1px; padding: .25rem .5rem }
            .markdown pre { padding: 0.3em; }
            .demo { background: white; padding: 1rem; border-width: 1px; border-radius: .25rem; border-color: #d8dee9; }
            .demo li { list-style-position: inside; }
            .demo button { padding: 0 .5rem; border-width: 1px; border-radius: .25rem; border-color: #a0aec0; background-color: #e2e8f0; }
            .demo button:hover { background-color: #edf2f7; }
            .demo input, .demo textarea, .demo select { padding: .25rem .5rem; border-width: 1px; border-radius: .25rem; border-color: #cbd5e0; }
            .demo input[type="checkbox"] { padding: 0; }
            .demo input[type="radio"] { padding: 0; border-radius: 999px; }
            .demo select { padding-right: 2.5rem; }"##
        }
                </style>
                <div
                    class="m-auto max-w-3xl px-6 pb-24 text-gray-800 antialiased markdown"
                >
                "__MARKDOWN_CONTENT_HERE__"
                <script src="https://giscus.app/client.js"
                    data-repo="geofmureithi/hirola"
                    data-repo-id="R_kgDOHvfVtA"
                    data-category="General"
                    data-category-id="DIC_kwDOHvfVtM4CQ9Y7"
                    data-mapping="og:title"
                    data-strict="0"
                    data-reactions-enabled="1"
                    data-emit-metadata="0"
                    data-input-position="bottom"
                    data-theme="light"
                    data-lang="en"
                    crossorigin="anonymous"
                    async=true>
                </script>
                </div>
            </main>
        </div>
        </body>
        </html>
    }
}

fn main() {
    use glob::glob;
    for entry in glob("src/pages/**/*.md").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                let (content, seo) = markdown_page(&path);
                let mut layout = "<!DOCTYPE html>".to_string();
                layout.push_str(&render_to_string(with_layout(seo)).unwrap());
                let html_path = path
                    .to_string_lossy()
                    .replace("src/pages", "dist")
                    .replace(".md", ".html");
                std::fs::create_dir_all("dist/basics").unwrap();
                std::fs::create_dir_all("dist/advanced").unwrap();
                std::fs::create_dir_all("dist/plugins").unwrap();
                let _file = File::create(&html_path).unwrap();
                std::fs::write(
                    &html_path,
                    layout.replace("__MARKDOWN_CONTENT_HERE__", &content),
                )
                .unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }
}

fn markdown_page(path: &PathBuf) -> (String, Seo) {
    let adapter = SyntectAdapter::new("InspiredGitHub");
    use comrak::ComrakOptions;
    let markdown = std::fs::read_to_string(path).unwrap();
    let mut options = ComrakOptions::default();
    let mut plugins = ComrakPlugins::default();

    options.extension.front_matter_delimiter = Some("---".to_owned());
    plugins.render.codefence_syntax_highlighter = Some(&adapter);
    let data = fronma::parser::parse::<Seo>(&markdown).unwrap();
    let res = markdown_to_html_with_plugins(data.body, &options, &plugins);
    (res, data.headers)
}
