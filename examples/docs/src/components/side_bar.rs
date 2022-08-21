use hirola::prelude::*;

pub fn SideBar(router: Router) -> Dom {
    html! {
        <ul class="space-y-2 text-gray-800">
                <li class="-ml-6 border-l-4 border-gray-700 pl-5 font-semibold">
                  <a mixin:route=&router.link() href="/" class="hover:text-gray-900">"Home"</a>
                </li>
                <li>
                  <span class="font-medium">"Basics"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/getting-started" class="hover:text-gray-900">"Installation"</a>
                    </li>
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/reactivity" class="hover:text-gray-900">"Reactivity"</a>
                    </li>
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/templating" class="hover:text-gray-900">"Templating"</a>
                    </li>
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/events" class="hover:text-gray-900">"Event Handling"</a>
                    </li>
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/iteration" class="hover:text-gray-900">"Iteration"</a>
                    </li>
                    <li class="">
                      <a mixin:route=&router.link() href="/basics/mixins" class="hover:text-gray-900">"Mixins"</a>
                    </li>
                  </ul>
                </li>
                <li>
                  <span class="font-medium">"Inbuilt Mixins"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/show" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"show" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/text" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"text" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/rhtml" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"rhtml"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/model" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"model" </a>
                    </li>

                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/transition" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"transition" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/ignore" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"ignore" </a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/mixins/if" class="hover:text-gray-900">
                        <span class="text-orange-500">"mixin:"</span>"if"</a>
                    </li>

                  </ul>
                </li>
                <li>
                  <span class="font-medium">"Plugins"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin::route=&router.link() href="/plugins/router" class="hover:text-gray-900">
                        "Router"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/plugins/form" class="hover:text-gray-900">"Form"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/plugins/global-state" class="hover:text-gray-900">
                        "Store"</a>
                    </li>

                  </ul>
                </li>
                <li>
                  <span class="font-medium">"Advanced"</span>
                  <ul class="pl-3">
                    <li class="">
                      <a mixin::route=&router.link() href="/advanced/testing" class="hover:text-gray-900">"Testing"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/advanced/extending" class="hover:text-gray-900">"Extending"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/advanced/async" class="hover:text-gray-900">"Async"</a>
                    </li>
                    <li class="">
                      <a mixin::route=&router.link() href="/advanced/ssr" class="hover:text-gray-900">"SSR"</a>
                    </li>
                  </ul>
                </li>
              </ul>
    }
}
