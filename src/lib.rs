
pub use hirola_core:: {
    node::Node,
    Dom,
    mount,
    events,
    render,
    wasm_bindgen,
    component::{
        Component,
        State,
        Props
    },
};

pub mod prelude {
    pub use hirola_core:: {
        node::Node,
        Dom,
        mount,
        component::{
            Component,
            State,
            Props
        },
    };
    pub use hirola_core::wasm_bindgen;
}