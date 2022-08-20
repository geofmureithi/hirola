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
