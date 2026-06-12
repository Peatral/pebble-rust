use crate::graphics::types::Color;
use pebble_sys::GCompOp;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Context {
    internal: *mut pebble_sys::GContext,
}

impl Context {
    pub fn as_ptr(&self) -> *mut pebble_sys::GContext {
        self.internal
    }

    pub fn set_stroke_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_stroke_color(self.internal, color.0);
        }
    }
    pub fn set_fill_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_fill_color(self.internal, color.0);
        }
    }
    pub fn set_text_color(&self, color: Color) {
        unsafe {
            pebble_sys::graphics_context_set_text_color(self.internal, color.0);
        }
    }
    pub fn set_compositing_mode(&self, mode: GCompOp) {
        unsafe {
            pebble_sys::graphics_context_set_compositing_mode(self.internal, mode);
        }
    }
    pub fn set_antialiased(&self, enable: bool) {
        unsafe {
            pebble_sys::graphics_context_set_antialiased(self.internal, enable);
        }
    }
    pub fn set_stroke_width(&self, stroke_width: u8) {
        unsafe { pebble_sys::graphics_context_set_stroke_width(self.internal, stroke_width) }
    }
}

impl From<*mut pebble_sys::GContext> for Context {
    fn from(internal: *mut pebble_sys::GContext) -> Self {
        Self { internal }
    }
}
