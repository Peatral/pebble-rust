use crate::layer::{ILayer, ILayerMut};
use pebble_sys::GCompOp;
use crate::graphics::bitmap::{BitmapMut, IBitmapMut};
use crate::graphics::types::Rect;

#[repr(transparent)]
pub struct BitmapLayer {
    internal: *mut pebble_sys::BitmapLayer,
}

impl BitmapLayer {
    pub fn new(bounds: Rect) -> BitmapLayer {
        unsafe {
            let internal = pebble_sys::bitmap_layer_create(bounds.0);

            BitmapLayer { internal }
        }
    }

    pub fn set_bitmap(&self, bitmap: BitmapMut) {
        unsafe {
            pebble_sys::bitmap_layer_set_bitmap(self.internal, bitmap.as_mut_ptr());
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
