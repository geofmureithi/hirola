#[macro_use]
extern crate validator_derive;

use serde::{Deserialize, Serialize};
use validator::Validate;

use hirola::{
    form::{Bind, FormHandler},
    prelude::{mixins::text, *},
};
use web_sys::{Event, HtmlInputElement};

#[derive(Validate, PartialEq, Clone, Serialize, Deserialize, Debug)]
struct Login {
    #[validate(length(min = 1, message = "Email is required"))]
    email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,

    count: u32,

    remember: String,
}

#[component]
fn InnerComponent(bind: Bind<u32, Login>) -> Dom {
    let increment = bind.callback(move |bind, _e: Event| {
        let value = bind.get_value();
        bind.set_value(value.get() + 1)
    });
    html! {
        <>
            <span>"Counter"</span>
            <button type="button" on:click=increment>"+"</button>
            <span mixin::text={&text(&bind.get_value())}></span>
        </>
    }
}

fn form_demo(_app: &App<S, G>) -> Dom {
    let form = FormHandler::new(Login {
        email: "example@gmail.com".to_string(),
        password: String::new(),
        count: 100,
        remember: "true".to_string(),
    });

    html! {
        <form
            class="h-screen flex flex-col items-center justify-center"
            method="post"
            ref={form.node_ref()}
            on:submit=|e| e.prevent_default()
            >
            <div class="mb-6">
                <label for="email"
                     class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">"Your email"</label>
                <input
                    type="email"
                    id="email"
                    name="email"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="name@example.com"

                    mixin:form={&form.register::<HtmlInputElement>()}
                    />
                <span class="text-red-700 text-sm"
                    mixin:text=&text(&form.error_for("email"))
                ></span>


            </div>
            <div class="mb-6">
                <label
                    for="password"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">"Your password"</label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required=""
                    mixin:form={&form.register::<HtmlInputElement>()}
                />
            </div>
            <InnerComponent bind={form.bind::<u32>("count")} />

            <div class="flex items-start mb-6">

                <div class="flex items-center h-5">
                    <input
                        id="remember"
                        name="remember"
                        type="checkbox"
                        value=""
                        class="w-4 h-4 bg-gray-50 rounded border border-gray-300 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800"
                        required=""
                        mixin:form={&form.register::<HtmlInputElement>()}
                    />
                </div>
                <label
                    x:text=""
                    x:class=""
                    for="remember"
                    class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                    "Remember me"
                </label>
            </div>
            <button
                type="submit"
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                >
                    "Submit"
            </button>
        </form>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let app = App<S, G>::new();
    app.mount(&body, form_demo);
}
