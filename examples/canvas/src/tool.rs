use hirola::prelude::*;
use hirola::dom::node_ref::NodeRef;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::Event;
use web_sys::MouseEvent;

#[derive(Clone)]
pub struct SignTool {
    pub(crate) is_mouse_clicked: Mutable<bool>,
    pub(crate) is_mouse_in_canvas: Mutable<bool>,
    pub(crate) prev_x: Mutable<i32>,
    pub(crate) cur_x: Mutable<i32>,
    pub(crate) prev_y: Mutable<i32>,
    pub(crate) cur_y: Mutable<i32>,
    pub(crate) canvas: NodeRef,
}

impl SignTool {
    pub fn new(canvas: NodeRef) -> Self {
        SignTool {
            is_mouse_clicked: Mutable::new(false),
            is_mouse_in_canvas: Mutable::new(false),
            prev_x: Mutable::new(0),
            cur_x: Mutable::new(0),
            prev_y: Mutable::new(0),
            cur_y: Mutable::new(0),
            canvas,
        }
    }

    pub fn update_position(&self, event: Event) {
        let e: MouseEvent = event.dyn_into().unwrap();
        let canvas = self
            .canvas
            .get()
            .inner_element()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        self.prev_x.set(self.cur_x.get());
        self.prev_y.set(self.cur_y.get());
        self.cur_x.set(e.client_x() - canvas.offset_left());
        self.cur_y.set(e.client_y() - canvas.offset_top());
    }

    pub fn callback<F, E>(&self, f: F) -> Box<dyn Fn(E)>
    where
        F: Fn(Self, E) + 'static,
    {
        let state = self.clone();
        let cb = move |e: E| {
            f(state.clone(), e);
        };
        Box::new(cb)
    }

    pub fn draw(&self) {
        let canvas = self
            .canvas
            .get()
            .inner_element()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.begin_path();
        context.move_to((self.prev_x.get()).into(), (self.prev_y.get()).into());
        context.line_to((self.cur_x.get()).into(), (self.cur_y.get()).into());
        context.set_stroke_style(&JsValue::from_str("black"));
        context.set_line_width(2.0);
        context.stroke();
        context.close_path();
    }
}
