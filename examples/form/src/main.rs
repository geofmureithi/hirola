use hirola::prelude::*;

#[derive(Validate, PartialEq, Clone)]
struct Login {
    #[validate(length(min = 1, message = "Email is required"))]
    email: String,
    #[validate(length(min = 1, message = "Password is required"))]
    password: String,
}

fn form_demo(_app: &HirolaApp) -> Dom {
    let form = FormHandler {
        inner: Signal::new(None),
        inputs: Signal::new(Vec::new()),
        value: Login {
            email: String::new(),
            password: String::new(),
        },
    };

    let connect = form.connect();

    html! {
        <form
            class="h-screen flex flex-col items-center justify-center"
            mixins=vec![connect.clone()]
            >
            <div class="mb-6">
                <label for="email"
                     class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">"Your email"</label>
                <input
                    type="email"
                    id="email"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    placeholder="name@example.com"
                    required=""
                    mixins=vec![register(&form.clone())]
                    />
            </div>
            <div class="mb-6">
                <label
                    for="password"
                    class="block mb-2 text-sm font-medium text-gray-900 dark:text-gray-300">"Your password"</label>
                <input
                    type="password"
                    id="password"
                    class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
                    required=""
                    // mixins=vec![register(&form.clone())]
                />
            </div>
            <div class="flex items-start mb-6">
            <div class="flex items-center h-5">
                <input id="remember"
                    type="checkbox"
                    value=""
                    class="w-4 h-4 bg-gray-50 rounded border border-gray-300 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800"
                    required=""
                    // mixins=vec![register(&form.clone())]
                />
                </div>
                <label
                    for="remember"
                    class="ml-2 text-sm font-medium text-gray-900 dark:text-gray-300">
                    "Remember me"
                </label>
            </div>
            <button
                type="submit"
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                // mixins=vec![(&form.clone()).register()]
                >
                    "Submit"
            </button>
        </form>
    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", form_demo);
}
