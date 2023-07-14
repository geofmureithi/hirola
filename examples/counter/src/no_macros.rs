use discard::DiscardOnDrop;
use hirola::prelude::{mixins::text, *};
use web_sys::{window, Event};
fn counter(_app: &App<S, G>) -> Dom {
    let count = Signal::new(0);
    let todos = MutableVec::<u32>::new_with_values(vec![0, 1, 2]);
    let alert = count
        .signal_ref(move |val| {
            // if val == &0 {
            //     return;
            // }
            window()
                .unwrap()
                .alert_with_message(&{
                    let res = ::std::fmt::format(format_args!("Count is at {0}", val));
                    res
                })
                .unwrap();
        })
        .to_future();
    let cb = count.clone();
    let fut = async move {
        // let _ = cb.set(2);
    };
    // let increment = count.mut_callback(|val, _e| val + 1);
    // let decrement = count.mut_callback(|val, _e| val - 1);
    ::hirola::prelude::TemplateResult::from({
        let element: ::hirola::prelude::DomType = ::hirola::prelude::GenericNode::element("div");
        // // let mut effects = Vec::new();
        // let node ={
        //     let res = Indexed {
        //         props: {
        //             IndexedProps {
        //                 iterable: todos.clone(),
        //                 template: move |todo| {
        //                     ::hirola::prelude::TemplateResult::from({
        //                         let element: ::hirola::prelude::DomType = ::hirola::prelude::GenericNode::element(
        //                             "span",
        //                         );
        //                         let mut effects = Vec::new();
        //                         ::hirola::prelude::GenericNode::append_render(
        //                             &element,
        //                             ::std::boxed::Box::new({ todo.to_string() }),
        //                         );
        //                         (element, effects)
        //                     })
        //                 },
        //             }
        //         },
        //     };
        //     let tpl = res.render();

        //     tpl
        // };
        // ::hirola::prelude::GenericNode::append_child(&element, &node.inner_element());
        let fut_drop = spawn(SideEffect::effect(alert));
        (element, vec![DiscardOnDrop::leak(fut_drop)])
    })
}

// let component = {
    //     let element: DomNode = GenericNode::element("a");
    //     element.set_attribute("href", "/counter/1");
    //     let component = TemplateResult::new(element);
    //     Mixin::<Identity>::mixin(&router.link(), &component);
    //     component.append_child(TemplateResult::new(GenericNode::text_node("Counter 1")));
    //     component
    // };
    // template.append_child(component);
    // template
    
fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let mut app = App<S, G>::new();

    let mut router = Router::new();
    router.add("/", |app| "Test".to_owned().render());
    router.add("/counter/1", counter);
    router.add("/counter/2", counter);

    fn main_view(app: &App<S, G>) -> Dom {
        let router = app.data::<Router>().unwrap().clone();
        let app = app.clone();

        let disp = NodeRef::new();
        let disp_cl = disp.clone();

        let renderer = router
            .render(&app)
            .signal_ref(move |dom| {
                let node = disp_cl.get::<DomNode>();
                let node = node.dyn_into::<web_sys::HtmlElement>().unwrap();
                node.replace_children_with_node_1(&dom.inner_element().inner_element());
            })
            .to_future();
        ::hirola::prelude::TemplateResult::from({
            let element: ::hirola::prelude::DomType =
                ::hirola::prelude::GenericNode::element("div");
            ::hirola::prelude::NodeRef::set(&disp, ::std::clone::Clone::clone(&element));
            let fut_drop = spawn(SideEffect::effect(renderer));
            (element, vec![DiscardOnDrop::leak(fut_drop)])
        })
    }
    app.extend(router);
    app.mount(&body, main_view);
}
