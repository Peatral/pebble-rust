use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::internal::types::{GBitmap, GBitmapFormat, GCornerMask, GOvalScaleMode};
use crate::types::{GColor, GCompOp, GPoint, GRect, GSize};
use core::ffi::c_int;

pub struct GContext {
    internal: *mut types::GContext,
}

impl GContext {
    pub(crate) fn from_ptr(ptr: *mut types::GContext) -> GContext {
        GContext { internal: ptr }
    }

    pub fn as_ptr(&self) -> *mut types::GContext {
        self.internal
    }

    pub fn set_stroke_color(&self, color: GColor) {
        interface::graphics_context_set_stroke_color(self.internal, color);
    }
    pub fn set_fill_color(&self, color: GColor) {
        interface::graphics_context_set_fill_color(self.internal, color);
    }
    pub fn set_text_color(&self, color: GColor) {
        interface::graphics_context_set_text_color(self.internal, color);
    }
    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::graphics_context_set_compositing_mode(self.internal, mode);
    }
    pub fn set_antialiased(&self, enable: bool) {
        interface::graphics_context_set_antialiased(self.internal, enable);
    }
    pub fn set_stroke_width(&self, stroke_width: u8) {
        interface::graphics_context_set_stroke_width(self.internal, stroke_width)
    }
}
