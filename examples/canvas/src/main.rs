use hirola::dom::effects::prelude::*;
use hirola::dom::{node_ref::NodeRef, Dom};
use hirola::prelude::*;
use tool::SignTool;
mod tool;

fn signature_pad() -> Dom {
    let canvas = NodeRef::new();
    let tool = SignTool::new(canvas.clone());

    let mouse_leave = tool.callback(|tool, _| {
        tool.is_mouse_in_canvas.set(false);
    });
    let mouse_up = tool.callback(|tool, _| {
        tool.is_mouse_clicked.set(false);
    });

    let mouse_move = tool.callback(|tool, e| {
        if tool.is_mouse_clicked.get() && tool.is_mouse_in_canvas.get() {
            tool.update_position(e);
            tool.draw();
        }
    });

    let mouse_down = tool.callback(move |tool, e| {
        tool.is_mouse_clicked.set(true);
        tool.update_position(e);
    });

    let mouse_enter = tool.callback(move |tool, e| {
        tool.is_mouse_in_canvas.set(true);
        tool.update_position(e);
    });
    html! {
        <canvas
            bind:ref=canvas
            width="500"
            height="300"
            style="position: absolute;border: 2px solid;"
            on:mouse_enter=mouse_enter
            on:mouse_out=mouse_leave
            on:mouse_down=mouse_down
            on:mouse_up=mouse_up
            on:mouse_move=mouse_move
        ></canvas>
    }
}

fn main() {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let root = hirola::dom::render_to(signature_pad(), &body).unwrap();
    std::mem::forget(root)
}
