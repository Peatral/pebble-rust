#![crate_type="staticlib"]
#![no_std]
#![no_builtins]

#[macro_use]
extern crate pebble_rust as pebble;

use core::cell::RefCell;
use pebble::{app, window_stack};
use pebble::window::{Window, WindowDelegate, WindowRef};
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{GRect, GPoint, GSize, GTextAlignment};

struct HelloDelegate {
    text_layer: RefCell<Option<TextLayer>>,
}

impl WindowDelegate for HelloDelegate {
    fn load(&self, window: WindowRef) {
        let root = window.get_root_layer();
        let bounds = root.get_bounds();

        let window_width = bounds.size.w;
        let window_height = bounds.size.h;

        let text_bounds = GRect {
            origin: GPoint { x: 0, y: window_height / 2 - 20 },
            size: GSize { w: window_width, h: 40 },
        };

        // We can print whatever we want.
        pbl_log!(c"This works like a %s, I can print numbers like %d", c"printf".as_ptr(), 25);

        // Or we can use other logging levels.
        pbl_warn!(c"This is a warning.");
        pbl_err!(c"Oops, something went wrong.");

        let text = TextLayer::new(text_bounds);

        text.set_text(c"Hello from Rust!");
        text.set_font(pebble::system::fonts::Font::get_system(c"RESOURCE_ID_ROBOTO_CONDENSED_21"));
        text.set_text_alignment(GTextAlignment::Center);

        root.add_child(&text);

        *self.text_layer.borrow_mut() = Some(text);
    }

    fn unload(&self, window: WindowRef) {
    }
}

#[unsafe(no_mangle)]
pub fn main() -> isize {
    pbl_log!(c"Loading app...");

    let app = app::App::new();

    let delegate = HelloDelegate {
        text_layer: RefCell::new(None),
    };
    let window = Window::new(delegate);

    window_stack::push(&window, false);
    app.run_event_loop();

    pbl_log!(c"Exiting...");

    0
}