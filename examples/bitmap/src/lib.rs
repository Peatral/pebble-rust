#![crate_type = "staticlib"]
#![no_std]
#![no_builtins]

extern crate pebble_rs as pebble;

use core::cell::RefCell;
use pebble::graphics::bitmap::Bitmap;
use pebble::layer::{BitmapLayer, ILayer, ILayerMut};
use pebble::window::{Window, WindowDelegate, WindowRef};
use pebble::{app, include_message_keys, window_stack};
use pebble_sys::GCompOp;

include_message_keys!();

struct BitmapExampleDelegate {
    bitmap: RefCell<Option<Bitmap>>,
    bitmap_layer: RefCell<Option<BitmapLayer>>,
}

impl WindowDelegate for BitmapExampleDelegate {
    fn load(&self, window: WindowRef) {
        let root = window.get_root_layer();
        let bounds = root.get_bounds();

        let bitmap = Bitmap::new(1);

        let bitmap_layer = BitmapLayer::new(bounds);
        bitmap_layer.set_bitmap(bitmap.as_mut());
        bitmap_layer.set_compositing_mode(GCompOp::GCompOpSet);

        root.add_child(&bitmap_layer);

        *self.bitmap.borrow_mut() = Some(bitmap);
        *self.bitmap_layer.borrow_mut() = Some(bitmap_layer);
    }

    fn unload(&self, _window: WindowRef) {
        self.bitmap.borrow_mut().take();
        self.bitmap_layer.take();
    }
}

#[unsafe(no_mangle)]
pub fn main() -> isize {
    let app = app::App::new();

    let delegate = BitmapExampleDelegate {
        bitmap: RefCell::new(None),
        bitmap_layer: RefCell::new(None),
    };
    let window = Window::new(delegate);

    window_stack::push(*window, false);

    app.run_event_loop();

    0
}
