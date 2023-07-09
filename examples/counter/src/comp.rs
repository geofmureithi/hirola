use hirola::prelude::*;
use web_sys::{window, Event};

#[derive(Debug, Clone)]
struct App {
    router: Router<Self>,
    numbers: MutableVec<i32>,
}


impl App {
    fn new(router: Router<Self>) -> Self {
        Self {
            router,
            numbers: MutableVec::new(),
        }
    }
    fn mount(self, parent: &web_sys::Node) {
        let display = NodeRef::new();
        let node: NodeRef<DomType> = display.clone();
        let router = self.router.clone();
        let renderer = router
            .render(self)
            .signal_ref(move |dom| {
                let node = node.get::<DomNode>();
                let node = node.dyn_into::<web_sys::HtmlElement>().unwrap();
                node.replace_children_with_node_1(&dom.inner_element().inner_element());
            })
            .to_future();
        render_to(
            ::hirola::prelude::TemplateResult::from({
                let template: ::hirola::prelude::TemplateResult<
                    ::hirola::prelude::DomType,
                > = ::hirola::prelude::TemplateResult::element("main");
                {
                    let effect = discard::DiscardOnDrop::leak(
                        spawn(hirola::prelude::SideEffect::effect(renderer)),
                    );
                    ::hirola::prelude::TemplateResult::effect(&template, effect);
                }
                ::hirola::prelude::NodeRef::set(
                    &display,
                    ::std::clone::Clone::clone(&template.inner_element()),
                );
                template
            }),
            parent,
        );
    }
}
fn counter(app: &App) -> Dom {
    let router = &app.router;
    let numbers = app.numbers.clone();
    // let add_one = numbers
    //     .update_with(|numbers, _e| {
    //         let len: i32 = numbers.lock_ref().len().try_into().unwrap();
    //         numbers.lock_mut().push(len + 1);
    //     });
    ::hirola::prelude::TemplateResult::from({
        let template: ::hirola::prelude::TemplateResult<::hirola::prelude::DomType> = ::hirola::prelude::TemplateResult::element(
            "div",
        );
        ::hirola::prelude::TemplateResult::append_child(
            &template,
            {
                {
                    let template: ::hirola::prelude::TemplateResult<
                        ::hirola::prelude::DomType,
                    > = ::hirola::prelude::TemplateResult::element("ul");
                    let template = {
                        let props = ::hirola::prelude::IndexedProps {
                            iterable: MutableVec::new_with_values((0..10).collect()),
                            holder: template,
                            template: move |item| {
                                {
                                    ::hirola::prelude::TemplateResult::from({
                                        let template: ::hirola::prelude::TemplateResult<
                                            ::hirola::prelude::DomType,
                                        > = ::hirola::prelude::TemplateResult::element("li");
                                        ::hirola::prelude::TemplateResult::append_child(
                                            &template,
                                            { item }.render(),
                                        );
                                        ::hirola::prelude::TemplateResult::event(
                                            &template,
                                            "click",
                                            ::std::boxed::Box::new(move |_| {
                                                let lvl = ::log::Level::Info;
                                                if lvl <= ::log::STATIC_MAX_LEVEL
                                                    && lvl <= ::log::max_level()
                                                {
                                                    ::log::__private_api_log(
                                                        format_args!("Clicked {0}", item),
                                                        lvl,
                                                        &(
                                                            "counter",
                                                            "counter",
                                                            "examples/counter/src/main.rs",
                                                            50u32,
                                                        ),
                                                        ::log::__private_api::Option::None,
                                                    );
                                                }
                                            }),
                                        );
                                        template
                                    })
                                }
                            },
                        };
                        let indexed = ::hirola::prelude::Indexed {
                            props,
                        };
                        indexed.render()
                    };
                    template
                }
            },
        );
        template
    })
}
fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let mut router = Router::new();
    router.add("/", counter);
    let app = App::new(router);
    app.mount(&body);
}