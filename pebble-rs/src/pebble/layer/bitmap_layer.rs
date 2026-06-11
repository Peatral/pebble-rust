use crate::layer::{ILayer, ILayerMut};
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::types::{Bitmap, GCompOp, GRect};

#[repr(transparent)]
pub struct BitmapLayer {
    internal: *mut types::BitmapLayer,
}

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        let internal = interface::bitmap_layer_create(bounds);

        BitmapLayer { internal }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}

impl ILayer for BitmapLayer {
    fn as_ptr(&self) -> *const types::Layer {
        interface::bitmap_layer_get_layer(self.internal)
    }
}

impl ILayerMut for BitmapLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        interface::bitmap_layer_get_layer(self.internal)
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        interface::bitmap_layer_destroy(self.internal);
    }
}
