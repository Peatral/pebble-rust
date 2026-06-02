#![crate_type="staticlib"]
#![no_std]
#![no_builtins]

extern crate pebble_rust as pebble;

use core::cell::RefCell;
use pebble::{app, window_stack};
use pebble::layer::{ILayer, BitmapLayer};
use pebble::types::{Bitmap, GCompOp};
use pebble::window::{Window, WindowDelegate, WindowRef};

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
        bitmap_layer.set_bitmap(&bitmap);
        bitmap_layer.set_compositing_mode(GCompOp::GCompOpSet);

        root.add_child(&bitmap_layer);

        *self.bitmap.borrow_mut() = Some(bitmap);
        *self.bitmap_layer.borrow_mut() = Some(bitmap_layer);
    }

    fn unload(&self, _window: WindowRef) {
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

    window_stack::push(window.as_ref(), false);

    app.run_event_loop();

    0
}