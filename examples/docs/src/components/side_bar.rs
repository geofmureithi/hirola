use hirola::prelude::*;

#[component]
pub fn SideBar() -> Dom {
    html! {
        <ul class="space-y-2 text-gray-800">
            <li class="-ml-6 border-l-4 border-gray-700 pl-5 font-semibold">
                <a href="/" class="hover:text-gray-900">
                    "Home"
                </a>
            </li>
            <li>
                <span class="font-medium">"Basics"</span>
                <ul class="pl-3">
                    <li class="">
                        <a

                            href="/basics/getting-started.html"
                            class="hover:text-gray-900"
                        >
                            "Installation"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/basics/reactivity.html"
                            class="hover:text-gray-900"
                        >
                            "Reactivity"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/basics/templating.html"
                            class="hover:text-gray-900"
                        >
                            "Templating"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/basics/events.html"
                            class="hover:text-gray-900"
                        >
                            "Event Handling"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/basics/mixins.html"
                            class="hover:text-gray-900"
                        >
                            "Mixins"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/basics/state-management.html"
                            class="hover:text-gray-900"
                        >
                            "State Management"
                        </a>
                    </li>
                </ul>
            </li>
            <li>
                <span class="font-medium">"Plugins"</span>
                <ul class="pl-3">
                    <li class="">
                        <a

                            href="/plugins/router.html"
                            class="hover:text-gray-900"
                        >
                            "Router"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/plugins/form.html"
                            class="hover:text-gray-900"
                        >
                            "Form"
                        </a>
                    </li>
                </ul>
            </li>
            <li>
                <span class="font-medium">"Advanced"</span>
                <ul class="pl-3">
                    <li class="">
                        <a

                            href="/advanced/testing.html"
                            class="hover:text-gray-900"
                        >
                            "Testing 🚧"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/advanced/async.html"
                            class="hover:text-gray-900"
                        >
                            "Async"
                        </a>
                    </li>
                    <li class="">
                        <a

                            href="/advanced/ssr.html"
                            class="hover:text-gray-900"
                        >
                            "SSR 🚧"
                        </a>
                    </li>
                </ul>
            </li>
        </ul>
    }
}
