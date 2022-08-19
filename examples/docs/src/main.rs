use std::fmt::Display;

use hirola::prelude::*;
use hirola::prelude::mixins::text;

fn SeoTitle<T: Display + ?Sized>(title: &'static T) -> Dom {
  web_sys::window().unwrap().document().unwrap().set_title(&format!("{title}"));
  Dom::empty()
}



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

fn SideBar(router: Router) -> Dom {
    html! {
        <ul class="space-y-2 text-gray-800">

                <li class="-ml-6 border-l-4 border-gray-700 pl-5 font-semibold">
                  <a href="/start-here" class="hover:text-gray-900">"Start Here"</a>
                </li>
                <li>
                  <span class="font-medium">"Basics"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/installation" class="hover:text-gray-900">"Installation"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/reactivity" class="hover:text-gray-900">"Reactivity"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/templating" class="hover:text-gray-900">"Templating"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/events" class="hover:text-gray-900">"Event Handling"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/iteration" class="hover:text-gray-900">"Iteration"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/basics/mixins" class="hover:text-gray-900">"Mixins"</a>
                    </li>
                  </ul>
                </li>
                <li>
                  <span class="font-medium">"Inbuilt Mixins"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/show" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"show" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/text" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"text" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/rhtml" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"rhtml"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/model" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"model" </a>
                    </li>

                    <li mixin::route=&router.link() class="">
                      <a href="/mixins/transition" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"transition" </a>
                    </li>
                    <li mixin::route=&router.link() class="">
                      <a href="/mixins/ignore" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"ignore" </a>
                    </li>
                    <li mixin::route=&router.link() class="">
                      <a href="/mixin/if" class="hover:text-gray-900">
                        <span class="text-gray-300">"mixin:"</span>"if"</a>
                    </li>

                  </ul>
                </li>
                // <li>
                //   <span class="font-medium">Magics</span>
                //   <ul class="pl-3">
                //     <li class="">
                //       <a href="/magics/el" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>el </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/refs" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>refs </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/store" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>store </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/watch" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>watch </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/dispatch" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>dispatch </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/nextTick" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>nextTick </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/root" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>root </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/data" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>data </a>
                //     </li>
                //     <li class="">
                //       <a href="/magics/id" class="hover:text-gray-900">
                //         <span class="text-gray-300">$</span>id </a>
                //     </li>
                //   </ul>
                // </li>
                // <li>
                //   <span class="font-medium">UI</span>
                //   <ul class="pl-3"></ul>
                // </li>
                // <li>
                //   <span class="font-medium">Globals</span>
                //   <ul class="pl-3">
                //     <li class="">
                //       <a href="/globals/alpine-data" class="hover:text-gray-900">
                //         <span class="text-gray-300">Alpine.</span>data() </a>
                //     </li>
                //     <li class="">
                //       <a href="/globals/alpine-store" class="hover:text-gray-900">
                //         <span class="text-gray-300">Alpine.</span>store() </a>
                //     </li>
                //     <li class="">
                //       <a href="/globals/alpine-bind" class="hover:text-gray-900">
                //         <span class="text-gray-300">Alpine.</span>bind() </a>
                //     </li>
                //   </ul>
                // </li>

                // <li>
                //   <span class="font-medium">Plugins</span>
                //   <ul class="pl-3">
                //     <li class="">
                //       <a href="/plugins/mask" class="hover:text-gray-900">Mask</a>
                //     </li>
                //     <li class="">
                //       <a href="/plugins/intersect" class="hover:text-gray-900">Intersect</a>
                //     </li>
                //     <li class="">
                //       <a href="/plugins/persist" class="hover:text-gray-900">Persist</a>
                //     </li>
                //     <li class="">
                //       <a href="/plugins/focus" class="hover:text-gray-900">Focus</a>
                //     </li>
                //     <li class="">
                //       <a href="/plugins/collapse" class="hover:text-gray-900">Collapse</a>
                //     </li>
                //     <li class="">
                //       <a href="/plugins/morph" class="hover:text-gray-900">Morph</a>
                //     </li>
                //   </ul>
                // </li>

                // <li>
                //   <span class="font-medium">Advanced</span>
                //   <ul class="pl-3">
                //     <li class="">
                //       <a href="/advanced/reactivity" class="hover:text-gray-900">Reactivity</a>
                //     </li>
                //     <li class="">
                //       <a href="/advanced/extending" class="hover:text-gray-900">Extending</a>
                //     </li>
                //     <li class="">
                //       <a href="/advanced/async" class="hover:text-gray-900">Async</a>
                //     </li>
                //     <li class="">
                //       <a href="/advanced/csp" class="hover:text-gray-900">CSP</a>
                //     </li>
                //   </ul>
                // </li>
              </ul>
    }
}

fn docs(app: &HirolaApp) -> Dom {
    let router = app.data::<Router>().unwrap().clone();
    let app = app.clone();

    html! {
        <div>
        <header class="bg-white md:fixed md:left-0 md:right-0 md:top-0 md:z-30 md:h-[5rem]">
        <div class="flex items-center justify-between pt-3">
          <div class="py-0 pl-6 text-2xl font-semibold text-gray-800 hover:text-gray-900 md:w-64">
            <a href="/" class="flex items-center">
              <img src="/alpine_long.svg" class="w-[200px] md:w-[290px]" alt=""/>
            </a>
          </div>
          <div class="hidden items-center justify-end space-x-6 py-4 pr-6 text-gray-800 md:flex">
            <div>
              <div id="docsearch">
                <button type="button" class="DocSearch DocSearch-Button" aria-label="Search">
                  <span class="DocSearch-Button-Container">
                    <svg width="20" height="20" class="DocSearch-Search-Icon" viewBox="0 0 20 20">
                      <path d="M14.386 14.386l4.0877 4.0877-4.0877-4.0877c-2.9418 2.9419-7.7115 2.9419-10.6533 0-2.9419-2.9418-2.9419-7.7115 0-10.6533 2.9418-2.9419 7.7115-2.9419 10.6533 0 2.9419 2.9418 2.9419 7.7115 0 10.6533z" stroke="currentColor" fill="none" fill-rule="evenodd" stroke-linecap="round" stroke-linejoin="round"></path>
                    </svg>
                    <span class="DocSearch-Button-Placeholder">"Search"</span>
                  </span>
                </button>
              </div>
            </div>
            <a href="https://github.com/alpinejs/alpine" class="block hover:text-gray-600">
              <svg class="h-5 w-5 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                <title>"GitHub"</title>
                <path d="M10 0a10 10 0 0 0-3.16 19.49c.5.1.68-.22.68-.48l-.01-1.7c-2.78.6-3.37-1.34-3.37-1.34-.46-1.16-1.11-1.47-1.11-1.47-.9-.62.07-.6.07-.6 1 .07 1.53 1.03 1.53 1.03.9 1.52 2.34 1.08 2.91.83.1-.65.35-1.09.63-1.34-2.22-.25-4.55-1.11-4.55-4.94 0-1.1.39-1.99 1.03-2.69a3.6 3.6 0 0 1 .1-2.64s.84-.27 2.75 1.02a9.58 9.58 0 0 1 5 0c1.91-1.3 2.75-1.02 2.75-1.02.55 1.37.2 2.4.1 2.64.64.7 1.03 1.6 1.03 2.69 0 3.84-2.34 4.68-4.57 4.93.36.31.68.92.68 1.85l-.01 2.75c0 .26.18.58.69.48A10 10 0 0 0 10 0"></path>
              </svg>
            </a>
            <a href="https://twitter.com/Alpine_JS" class="block hover:text-gray-600">
              <svg class="h-5 w-5 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                <title>"Twitter"</title>
                <path d="M6.29 18.25c7.55 0 11.67-6.25 11.67-11.67v-.53c.8-.59 1.49-1.3 2.04-2.13-.75.33-1.54.55-2.36.65a4.12 4.12 0 0 0 1.8-2.27c-.8.48-1.68.81-2.6 1a4.1 4.1 0 0 0-7 3.74 11.65 11.65 0 0 1-8.45-4.3 4.1 4.1 0 0 0 1.27 5.49C2.01 8.2 1.37 8.03.8 7.7v.05a4.1 4.1 0 0 0 3.3 4.03 4.1 4.1 0 0 1-1.86.07 4.1 4.1 0 0 0 3.83 2.85A8.23 8.23 0 0 1 0 16.4a11.62 11.62 0 0 0 6.29 1.84"></path>
              </svg>
            </a>
            <a href="https://alpinejs.codewithhugo.com/chat/" class="block hover:text-gray-600">
              <svg class="h-5 w-5 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 146 146">
                <title>"Discord"</title>
                <path d="M107.75 125.001s-4.5-5.375-8.25-10.125c16.375-4.625 22.625-14.875 22.625-14.875-5.125 3.375-10 5.75-14.375 7.375-6.25 2.625-12.25 4.375-18.125 5.375-12 2.25-23 1.625-32.375-.125-7.125-1.375-13.25-3.375-18.375-5.375-2.875-1.125-6-2.5-9.125-4.25-.375-.25-.75-.375-1.125-.625-.25-.125-.375-.25-.5-.375-2.25-1.25-3.5-2.125-3.5-2.125s6 10 21.875 14.75c-3.75 4.75-8.375 10.375-8.375 10.375-27.625-.875-38.125-19-38.125-19 0-40.25 18-72.875 18-72.875 18-13.5 35.125-13.125 35.125-13.125l1.25 1.5c-22.5 6.5-32.875 16.375-32.875 16.375s2.75-1.5 7.375-3.625c13.375-5.875 24-7.5 28.375-7.875.75-.125 1.375-.25 2.125-.25 7.625-1 16.25-1.25 25.25-.25 11.875 1.375 24.625 4.875 37.625 12 0 0-9.875-9.375-31.125-15.875l1.75-2S110 19.626 128 33.126c0 0 18 32.625 18 72.875 0 0-10.625 18.125-38.25 19zM49.625 66.626c-7.125 0-12.75 6.25-12.75 13.875s5.75 13.875 12.75 13.875c7.125 0 12.75-6.25 12.75-13.875.125-7.625-5.625-13.875-12.75-13.875zm45.625 0c-7.125 0-12.75 6.25-12.75 13.875s5.75 13.875 12.75 13.875c7.125 0 12.75-6.25 12.75-13.875s-5.625-13.875-12.75-13.875z" fill-rule="nonzero"></path>
              </svg>
            </a>
          </div>
          <div class="flex items-center pr-6 md:hidden" x-data="{ show: false }">
            <button class="text-gray-600 focus:outline-none" click="show = ! show">
              <span class="sr-only">"Show navigation"</span>
              <svg class="h-6 w-6 fill-current" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20">
                <path d="M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z"></path>
              </svg>
            </button>
            <div class="fixed top-0 bottom-0 right-0 z-[199] w-1/2 overflow-y-auto bg-gray-100 p-6 shadow-xl md:top-[4rem]" x-show="show">
              <div class="flex justify-end">
                <button class="text-gray-600 focus:outline-none" click="show = ! show">
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
                .demo select { padding-right: 2.5rem; }"##}
            </style>
            <div class="m-auto max-w-3xl px-6 pb-24 text-gray-800 antialiased markdown">
            {router.render(&app)}
            </div>
        </main>
      </div>
    }
}

const INDEX: &str = r#"<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Hirola Counter</title>
  </head>
</html>
"#;

fn main() {
    let mut app = HirolaApp::new();
    let mut router = Router::new();
    router.add("/", |_| {
        html!{
            <div>
                <h1>"Start here"</h1>
                <p>"Create a blank HTML file somewhere on your computer with a name like: "<code>"i-love-alpine.html"</code></p>
            </div>
        }
    });
    router.add("/basics/installation", |_| {

        html! {
            <div>
              <SeoTitle title={"Installation | Hirola"} />
                <h1>"Pre"</h1>
                <p>
                  "Before getting started with"
                  <code class="one-liner">"hirola"</code>
                  " we are going to assume that you have the following tools installed:"
                </p>
                <ul class="ml-4">
                  <li>"Rust"</li>
                  <li>"Cargo"</li>
                  <li>"Trunk"</li>
                </ul>
                <h1>"Getting Started"</h1>
                <p>"We are going to create a simple counter program."</p>
                <code class="block one-liner my-1 py-1">
                  "cargo new counter"
                </code>
                <p>"With a new project, we need to create an index file which is the entry point and required by trunk"</p>
                <code class="block one-liner my-1 py-1">
                  "cd counter"
                </code>
                <p>"Create an "<b>"index.html"</b>" in the root of counter. Add the contents below"</p>
              
                  <pre class="text-sm my-2">
                    <code class="language-html">
                    {INDEX}
                    </code>
                  </pre>
                  <p>"Lets add some code to "<b>"src/main.rs" </b></p>
                  <pre class="text-sm my-2">
                    <code>
                    {include_str!("../../counter/src/main.rs")}
                    </code>
                  </pre>
                  <p>"Now lets run our project"</p>
                  <code class="block one-liner my-1 py-1">
                    "trunk serve"
                  </code>
                  <p>"You should be able to get counter running."</p>
                  <p class="text-xs"><span>"Try it out"</span></p>
                  <div class="demo">
                  {
                      let count = Signal::new(0);
                      html! {
                          <div>
                            <button on:click={count.mut_callback(|c, _| c + 1)}>"Increment"</button>
                            <span class="ml-1">{count.get()}</span>
                          </div>
                        }
                    }
                  </div>
            </div>
        }
    });
    router.add("/basics/reactivity", |_| {
      html! {
          <div>
              <SeoTitle title={"Reactivity | Hirola"} />
              <h1>"Reactivity"</h1>
              <p>
              r#"Hirola offers reactivity via a primitive called signal and an effect called create_effect. Once a signal is updated, these changes are propagated to the dom."#
              </p>
              <blockquote>
              <p>"Hirola uses a fork of maple(now sycamore) reactivity engine under the hood to provide these functions."
                <a href="https://sycamore-rs.netlify.app/docs/basics/reactivity">"→ Read more about sycamore reactivity primitives"</a>
              </p>
            </blockquote>
            <h2>"Reactive Signal"</h2>
              <pre>
                <code>
                "use hirola_core::prelude::*;
let state = Signal::new(0);
assert_eq!(*state.get(), 0);
                
state.set(1);
assert_eq!(*state.get(), 1);"
                </code>
              </pre>
              
            <p>"Signal is pretty similar to useState in react or Alpine.reactive"</p>
            <h2>"Subscribing"</h2>
            <p>"Subscribing is done via create_effect"</p>
            <pre>
              <code>
"use hirola_core::prelude::*;
let state = Signal::new(0);
assert_eq!(*state.get(), 0);
create_effect(move || {
  let new_value = state.get();
  // do something
})
/// later
state.set(1);
"
</code>
</pre>
          </div>
      }
  });
    router.add("/basics/templating", |_| {
        html! {
            <div>
              <SeoTitle title={"Templating | Hirola"} />
              <h1>"Templating"</h1>
              <p>"Install blah blah "<code>"i-love-alpine.html"</code></p>
            </div>
        }
    });
    router.add("/basics/mixins", |_| {
      html! {
          <div>
              <h1>"Mixins"</h1>
              <p>"Mixins are ways of sharing and extending code in hirola."</p>
          </div>
      }
    });
    router.add("/basics/iteration", |_| {
      html! {
          <div>
              <h1>"Iteration"</h1>
              <p>"Install blah blah "<code>"i-love-alpine.html"</code></p>
          </div>
      }
  });
    router.add("/basics/events", |_| {
      html! {
          <div>
              <h1>"Event Handling"</h1>
              <p>"Hirola uses an "<code>"on:<event>"</code>" binding style"</p>
              <h2>"Example"</h2>
              <pre>
              <code>
r#"let clicked = Signal::new(false);
html! {
    <div>
      <button on:click={clicked.mut_callback(|c, _| !c)}>"Click Me"</button>
      <span>{format!("Clicked? {}", clicked.get())}</span>
    </div>
}"#
            </code>
            </pre>
              <div class="demo">
                  {
                      let clicked = Signal::new(false);
                      html! {
                          <div>
                            <button on:click={clicked.mut_callback(|c, _| !c)}>"Click Me"</button>
                            <span class="ml-1">{format!("Clicked? {}", clicked.get())}</span>
                          </div>
                        }
                    }
                  </div>
            
          </div>
      }
  });
  router.add("/mixins/:mixin", |_| {
    html! {
        <div>
            <h1>"Mixin"</h1>
            <p>"Install blah blah "<code>"i-love-alpine.html"</code></p>
        </div>
    }
});
    app.extend(router);
    app.mount("body", docs);
}
