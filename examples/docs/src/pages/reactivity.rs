use crate::components::code_predom::CodePredom;
use crate::components::seo_title::SeoTitle;
use crate::App;
use hirola::prelude::*;

pub fn reactivity_page(_app: &App) -> Dom {
    html! {
        <div>
            <SeoTitle title="Reactivity | Hirola"/>
            <h1>"Reactivity"</h1>
            <p>
                "Hirola offers reactivity via a mutable called signal and an effect called create_effect. Once a signal is updated, these changes are propagated to the dom."
            </p>
            <blockquote>
                <p>
                    "Hirola uses futures_signals crate as its reactivity base"
                    //docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html">"→ Read more about futures_signals"</a>
                    <a href="https://docs.rs/futures-signals/0.3.32/futures_signals/tutorial/index.html">
                        "→ Read more about futures_signals"
                    </a>
                </p>
            </blockquote>
            <h2>"Reactive Signal"</h2>
            <CodePredom
                code="use hirola_core::prelude::*;
                let state = Signal::new(0);
                assert_eq!(state.get(), 0);
                
                state.set(1);
                assert_eq!(state.get(), 1);"

                file="main.rs"
            />

            <p>"Signal is pretty similar to useState in react or Alpine.reactive"</p>
            <h2>"Subscribing"</h2>
            <p>"Subscribing is done via create_effect"</p>
            <CodePredom
                code="use hirola_core::prelude::*;
                let state = Signal::new(0);
                assert_eq!(state.get(), 0);
                create_effect(state.clone(), move |new_value| {
                // do something with new value
                });
                /// later
                state.set(1);"
                // do something with new value
                /// later
                file="main.rs"
            />

        </div>
    }
}
