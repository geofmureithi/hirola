#![recursion_limit = "256"]

pub extern crate typed_html;

pub mod component;
pub mod events;
pub mod reactive;

pub mod node;
mod props;

pub mod macros;

use crate::component::*;

pub use typed_html::*;

pub type Dom = dominator::Dom;

use std::sync::Arc;

pub use wasm_bindgen;



pub fn mount<C, S, P>(component: &mut C, props: P)
where
    C: Component<P, S>,
    S: State + Default,
    P: Props,
{
    let state = Arc::new(S::default());
    component.before_mount(&props, &state); // simulate props
    let render = component.render(&state);
    dominator::append_dom(&dominator::body(), render);
    component.on_mount(&props, &state);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
