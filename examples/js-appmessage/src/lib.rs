#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]

extern crate alloc;

#[macro_use]
extern crate pebble_rust as pebble;

use alloc::borrow::ToOwned;

use pebble::app_message::*;
use pebble::layer::{ILayerMut, ILayer, TextLayer};
use pebble::types::{GPoint, GRect, GSize, GTextAlignment, GlobalRefCell};
use pebble::window::{Window, WindowDelegate, WindowRef};
use pebble::{app, window_stack};

include_message_keys!();

static TEXT_LAYER: GlobalRefCell<Option<TextLayer>> = GlobalRefCell::new(None);

struct AppMessageDelegate;

impl WindowDelegate for AppMessageDelegate {
    fn load(&self, window: WindowRef) {
        pbl_log!(c"Window loaded at address %p", window.as_ptr());

        let root = window.get_root_layer();
        let bounds = root.get_bounds();

        let window_width = bounds.size.w;
        let window_height = bounds.size.h;

        let text_bounds = GRect {
            origin: GPoint {
                x: 0,
                y: window_height / 2 - 20,
            },
            size: GSize {
                w: window_width,
                h: 40,
            },
        };

        let text = TextLayer::new(text_bounds);
        text.set_text_static(c"Loading...");
        text.set_text_alignment(GTextAlignment::Center);
        root.add_child(&text);

        *TEXT_LAYER.borrow_mut() = Some(text);
    }

    fn unload(&self, _window: WindowRef) {
        TEXT_LAYER.borrow_mut().take();
    }
}

#[unsafe(no_mangle)]
pub fn main() -> isize {
    AppMessage::register_inbox_received(message_received);

    if AppMessage::open(200, 200).is_err() {
        pebble::pbl_err!(c"Failed to open AppMessage subsystem!");
    }

    let app = app::App::new();

    let delegate = AppMessageDelegate;
    let window = Window::new(delegate);

    window_stack::push(*window, false);

    app.run_event_loop();

    pbl_log!(c"Exiting.");

    0
}

fn message_received(dict: Dictionary) {
    if let Some(tuple) = dict.find(message_keys::MESSAGE_KEY_EXAMPLE) {
        if let Some(text_val) = tuple.get_string() {
            let new_str = text_val.to_owned();

            if let Some(layer) = TEXT_LAYER.borrow_mut().as_mut() {
                layer.set_text(new_str);
            }
        }
    }
}
