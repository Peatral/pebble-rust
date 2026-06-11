use crate::layer::{ILayer, ILayerMut};
use crate::system::fonts::Font;
use alloc::ffi::CString;
use core::ffi::CStr;
use pebble_sys::GTextAlignment;
use crate::graphics::types::Rect;

pub struct TextLayer {
    internal: *mut pebble_sys::TextLayer,
    _text_buffer: Option<CString>,
}

impl TextLayer {
    pub fn new(bounds: Rect) -> TextLayer {
        unsafe {
            let internal = pebble_sys::text_layer_create(bounds.0);

            TextLayer {
                internal,
                _text_buffer: None,
            }
        }
    }

    /// Sets the text using a static C-String.
    pub fn set_text_static(&self, text: &'static CStr) {
        unsafe {
            pebble_sys::text_layer_set_text(self.internal, text.as_ptr());
        }
    }

    /// Sets the text using a dynamically allocated CString.
    pub fn set_text(&mut self, text: CString) {
        unsafe {
            pebble_sys::text_layer_set_text(self.internal, text.as_ptr());
        }

        self._text_buffer = Some(text);
    }

    pub fn set_font(&self, font: Font) {
        unsafe { pebble_sys::text_layer_set_font(self.internal, font.internal) }
    }

    pub fn set_text_alignment(&self, alignment: GTextAlignment) {
        unsafe {
            pebble_sys::text_layer_set_text_alignment(self.internal, alignment);
        }
    }
}

impl ILayer for TextLayer {
    fn as_ptr(&self) -> *const pebble_sys::Layer {
        unsafe { pebble_sys::text_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for TextLayer {
    fn as_mut_ptr(&self) -> *mut pebble_sys::Layer {
        unsafe { pebble_sys::text_layer_get_layer(self.internal) }
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::text_layer_destroy(self.internal);
        }
    }
}
