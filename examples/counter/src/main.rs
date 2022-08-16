use hirola::{form::bind::bind_input, prelude::*};

fn counter(_app: &HirolaApp) -> Dom {
    let count = Signal::new(99);

    let decerement = count.mut_callback(|count, _e| *count - 1);

    let incerement = count.mut_callback(|count, _e| *count + 1);

    html! {
            <div class="grid h-screen place-items-center">
                <div
                    class="h-10 w-32">
                    <div class="flex flex-row h-10">
                        <button
                            on:click=decerement
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "-"
                        </button>
                        <div class="block">
                            <input
                                mixin::bind=&bind_input(&count)
                            />
                        </div>
                        <button
                            on:click=incerement
                            class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
                            "+"
                        </button>
                    </div>
                </div>
           </div>
    }
}

fn main() {
    let app = HirolaApp::new();
    app.mount("body", counter);
}
