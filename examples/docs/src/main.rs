mod components;
mod pages;

use components::logo::HirolaLogo;
use hirola::prelude::*;
use pages::{
    async_page, event_handling_page, extending_page, forms_page, getting_started_page, home,
    inner_mixins, mixins_page, reactivity_page, router_page, ssr_page, state_page, templating_page,
    testing_page,
};

use crate::components::side_bar::SideBar;

// macro_rules! make_example {

//      ($jsx:expr)=>{

//          {
//           // {
//           //   html! {
//           //     <pre><code>{stringify!($jsx)}</code></pre>
//           //   }
//           // };
//             html! {
//               <div class="demo">
//                 <pre>
//                 {std::stringify!($jsx)}
//                 </pre>
//               </div>
//             }
//          }
//      }
//  }

impl App {
    fn new(router: Router<Self>) -> Self {
        Self { router }
    }
    fn mount(&self, parent: &web_sys::Node) {
        let router = self.router.clone();

        let mount = NodeRef::new();
        let node: NodeRef<DomType> = mount.clone();
        let renderer = router
            .render(&self)
            .signal_ref(move |dom| {
                let node = node.get::<DomNode>();
                let node = node.dyn_into::<web_sys::HtmlElement>().unwrap();
                node.replace_children_with_node_1(&dom.inner_element().inner_element());
            })
            .to_future();

        render_to(
            html! {
              <div use:renderer>
              <header class="bg-white md:fixed md:left-0 md:right-0 md:top-0 md:z-30 md:h-[5rem]">
              <div class="flex items-center justify-between pt-3">
                <div class="py-0 pl-6 text-2xl font-semibold text-gray-800 hover:text-gray-900 md:w-64">
                  <a href="/" class="flex items-center">
                    <div class="w-[200px] md:w-[290px]">
                      <HirolaLogo />
                    </div>
                  </a>
                </div>
                <div class="hidden items-center justify-end space-x-6 py-4 pr-6 text-gray-800 md:flex">
                  <a href="https://github.com/geofmureithi/hirola" class="block hover:text-gray-600">
                    "GitHub"
                  </a>
                </div>
                <div class="flex items-center pr-6 md:hidden">
                  <button class="text-gray-600 focus:outline-none" >
                    <span class="sr-only">"Show navigation"</span>
                    <svg class="h-6 w-6 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                      <path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"></path>
                    </svg>
                  </button>
                  <div class="fixed top-0 bottom-0 right-0 z-[199] w-1/2 overflow-y-auto bg-gray-100 p-6 shadow-xl md:top-[4rem]" x-show="show">
                    <div class="flex justify-end">
                      <button class="text-gray-600 focus:outline-none">
                        <span class="sr-only">"Close navigation"</span>
                        <svg class="h-6 w-6" stroke="currentColor" fill="none" viewBox="0 0 24 24">
                          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                        </svg>
                      </button>
                    </div>
                    <SideBar router={(&router).clone()} />
                  </div>
                </div>
              </div>
            </header>
            <style>
              "[hover-scrollbar] { overflow-x: 'visible'; overflow-y: hidden; } [hover-scrollbar]:hover { overflow-y: auto }"
            </style>
            <aside class="fixed left-0 bottom-0 hidden w-48 px-8 pb-6 pt-8 md:top-[4rem] md:block lg:w-64" hover-scrollbar>
              <SideBar router={(&router).clone()} />
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
                    .demo { background: white; padding: 1rem; border-width: 1px; border-radius: .25rem; border-color: #d8dee9; }
                    .demo li { list-style-position: inside; }
                    .demo button { padding: 0 .5rem; border-width: 1px; border-radius: .25rem; border-color: #a0aec0; background-color: #e2e8f0; }
                    .demo button:hover { background-color: #edf2f7; }
                    .demo input, .demo textarea, .demo select { padding: .25rem .5rem; border-width: 1px; border-radius: .25rem; border-color: #cbd5e0; }
                    .demo input[type="checkbox"] { padding: 0; }
                    .demo input[type="radio"] { padding: 0; border-radius: 999px; }
                    .demo select { padding-right: 2.5rem; }"##.render()}
                  </style>
                  <div class="m-auto max-w-3xl px-6 pb-24 text-gray-800 antialiased markdown" ref=mount>

                  </div>
              </main>
            </div>
            },
            parent,
        );
    }
}

#[derive(Clone)]
pub struct App {
    router: Router<Self>,
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let mut router = Router::new();
    router.route("/", home);
    router.route("/basics/getting-started", getting_started_page);
    router.route("/basics/reactivity", reactivity_page);
    router.route("/basics/templating", templating_page);
    router.route("/basics/mixins", mixins_page);
    router.route("/basics/events", event_handling_page);

    router.route("/mixins/:mixin", inner_mixins);

    router.route("/advanced/testing", testing_page);
    router.route("/advanced/ssr", ssr_page);
    router.route("/advanced/async", async_page);
    router.route("/advanced/extending", extending_page);

    router.route("/plugins/form", forms_page);
    router.route("/plugins/router", router_page);
    router.route("/plugins/state", state_page);

    let app = App::new(router);
    app.mount(&body);
}
