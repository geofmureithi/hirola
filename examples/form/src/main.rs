#[macro_use]
extern crate validator_derive;

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use hirola::{
    dom::*,
    prelude::*,
    signal::{Signal, SignalExt},
    signal_vec::SignalVecExt,
};
use hirola_form::{Form, FormHandler};
use web_sys::window;

#[derive(Validate, PartialEq, Clone, Serialize, Deserialize, Debug, FormEntity)]
struct Login {
    #[validate(length(min = 1, message = "Email is required"))]
    email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
    remember: String,
}
impl Login {
    fn errors(&self) -> Option<BTreeMap<LoginForm, ValidationError>> {
        let res = Validate::validate(&self).err()?;
        Some(
            res.errors()
                .into_iter()
                .map(|(key, value)| {
                    (
                        (*key).parse::<LoginForm>().unwrap(),
                        match value {
                            validator::ValidationErrorsKind::Struct(_fields) => todo!(),
                            validator::ValidationErrorsKind::List(_) => todo!(),
                            validator::ValidationErrorsKind::Field(f) => {
                                f.clone().get(0).cloned().unwrap()
                            }
                        },
                    )
                })
                .collect(),
        )
    }
}

fn form_demo() -> Dom {
    let form = FormHandler::new(Login {
        email: "example@gmail.com".to_string(),
        password: String::new(),
        // count: 100,
        remember: "true".to_string(),
    });

    html! {
        <form
            class="h-screen flex flex-col items-center justify-center"
            method="post"
            bind:ref=form.node_ref()
            novalidate=""
            on:submit=move |e: web_sys::Event| {
                e.prevent_default();
                let value = form.current();
                window()
                    .unwrap()
                    .alert_with_message(&serde_json::to_string(&value).unwrap())
                    .unwrap();
            }
        >
            <div class="mb-6">
                <label
                    for="email"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                >
                    "Your email"
                </label>
                <input
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="name@example.com"
                    x:form=form.bind(LoginForm::Email)
                />
                <span
                    class="text-red-700 text-sm"
                    x:text=error_for(&form, LoginForm::Email).dedupe_cloned()
                ></span>

            </div>
            <div class="mb-6">
                <label
                    for="password"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                >
                    "Your password"
                </label>
                <input
                    type="password"
                    id="password"
                    name="password"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required=""
                    x:form=form.bind(LoginForm::Password)
                />
            </div>

            <span
                class="text-red-700 text-sm"
                x:text=error_for(&form, LoginForm::Password).dedupe_cloned()
            ></span>

            <div class="flex items-start mb-6">

                <div class="flex items-center h-5">
                    <input
                        id="remember"
                        name="remember"
                        type="checkbox"
                        value=""
                        class="w-4 h-4 bg-gray-50 rounded border border-gray-300 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800"
                        required=""
                        x:form=form.bind(LoginForm::Remember)
                    />
                </div>
                <label
                    for="remember"
                    class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300"
                >
                    "Remember me"
                </label>
            </div>
            <span
                class="text-red-700 text-sm"
                x:text=error_for(&form, LoginForm::Remember).dedupe_cloned()
            ></span>
            <button
                type="submit"
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
            >
                "Submit"
            </button>
        </form>
    }
}

fn errors_for(
    form: &FormHandler<Login>,
    column: LoginForm,
) -> impl Signal<Item = Vec<ValidationError>> {
    let f = form.clone();
    let errors = form
        .value
        .entries_cloned()
        .filter(move |e| &e.0 == &column)
        .filter_map(move |_e| {
            let current = f.current();
            current.errors()?.get(&column).cloned()
        })
        .to_signal_cloned();
    errors
}

fn error_for(form: &FormHandler<Login>, column: LoginForm) -> impl Signal<Item = String> {
    errors_for(&form, column).map(|items| items.get(0).map(|v| v.to_string()).unwrap_or_default())
}

fn main() {
    console_error_panic_hook::set_once();
    mount(form_demo()).unwrap();
}
