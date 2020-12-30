#[macro_export]
macro_rules! render {
    ($html:expr) => {
        let mut doc = html!(
            $html
        : Node);
        Node::build(&state, doc.vnode()).into_dom()
    };
}
