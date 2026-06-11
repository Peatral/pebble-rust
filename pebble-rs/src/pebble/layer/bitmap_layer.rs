use crate::layer::{ILayer, ILayerMut};
use crate::types::Bitmap;
use pebble_sys::GCompOp;
use crate::graphics::types::GRect;

#[repr(transparent)]
pub struct BitmapLayer {
    internal: *mut pebble_sys::BitmapLayer,
}

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        unsafe {
            let internal = pebble_sys::bitmap_layer_create(bounds.0);

            BitmapLayer { internal }
        }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        unsafe {
            pebble_sys::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
        }
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        unsafe {
            pebble_sys::bitmap_layer_set_compositing_mode(self.internal, mode);
        }
    }
}

impl ILayer for BitmapLayer {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        unsafe { pebble_sys::bitmap_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for BitmapLayer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        unsafe { pebble_sys::bitmap_layer_get_layer(self.internal) }
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::bitmap_layer_destroy(self.internal);
        }
    }
}
