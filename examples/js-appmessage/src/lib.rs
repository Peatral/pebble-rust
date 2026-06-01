#![crate_type="staticlib"]
#![no_std]
#![no_builtins]

extern crate alloc;

#[macro_use]
extern crate pebble_rust as pebble;

use alloc::borrow::ToOwned;
use alloc::ffi::CString;
use core::ffi::c_char;
use core::sync::atomic::{AtomicPtr, Ordering};
use pebble::{app, window_stack};
use pebble::app_message::*;
use pebble::layer::{ILayer, TextLayer};
use pebble::types::{GPoint, GRect, GSize, GTextAlignment};
use pebble::window::{Window, WindowDelegate, WindowRef};

const MESSAGE_KEY_EXAMPLE: u32 = 1768777472;

static mut TEXT_LAYER: Option<TextLayer> = None;
static mut SAVED_TEXT: Option<CString> = None;
static GLOBAL_CSTRING: AtomicPtr<c_char> = AtomicPtr::new(core::ptr::null_mut());

struct AppMessageDelegate;

impl WindowDelegate for AppMessageDelegate {
    fn load(&self, window: WindowRef) {
        pbl_log!(c"Window loaded at address %p", window.as_ptr());

        let root = window.get_root_layer();
        let bounds = root.get_bounds();

        let window_width = bounds.size.w;
        let window_height = bounds.size.h;

        let text_bounds = GRect {
            origin: GPoint { x: 0, y: window_height / 2 - 20 },
            size: GSize { w: window_width, h: 40 }
        };

        unsafe {
            let text = TextLayer::new(text_bounds);
            text.set_text(c"Loading...");
            text.set_text_alignment(GTextAlignment::Center);
            root.add_child(&text);

            TEXT_LAYER = Some(text);
        }
    }

    fn unload(&self, _window: WindowRef) {
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

    window_stack::push(&window, false);
    app.run_event_loop();

    pbl_log!(c"Exiting.");

    0
}

fn message_received(dict: Dictionary) {
    if let Some(tuple) = dict.find(MESSAGE_KEY_EXAMPLE) {
        if let Some(text_val) = tuple.get_string() {
            unsafe {
                if let Some(layer) = &*(&raw const TEXT_LAYER) {
                    // TODO: str is cleared when leaving the scope and the text vanishes.
                    // it needs to be stored globally
                    let str = text_val.to_owned();
                    layer.set_text(str.as_c_str());
                }
            }
        }
    }
}