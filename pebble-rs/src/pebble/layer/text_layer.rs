use crate::layer::{ILayer, ILayerMut};
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::system::fonts::Font;
use crate::types::{GRect, GTextAlignment};
use alloc::ffi::CString;
use core::ffi::CStr;

pub struct TextLayer {
    internal: *mut types::TextLayer,
    _text_buffer: Option<CString>,
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);

        TextLayer {
            internal,
            _text_buffer: None,
        }
    }

    /// Sets the text using a static C-String.
    pub fn set_text_static(&self, text: &'static CStr) {
        interface::text_layer_set_text(self.internal, text);
    }

    /// Sets the text using a dynamically allocated CString.
    pub fn set_text(&mut self, text: CString) {
        interface::text_layer_set_text(self.internal, &text);

        self._text_buffer = Some(text);
    }

    pub fn set_font(&self, font: Font) {
        interface::text_layer_set_font(self.internal, font.internal)
    }

    pub fn set_text_alignment(&self, alignment: GTextAlignment) {
        interface::text_layer_set_text_alignment(self.internal, alignment);
    }
}

impl ILayer for TextLayer {
    fn as_ptr(&self) -> *const types::Layer {
        interface::text_layer_get_layer(self.internal)
    }
}

impl ILayerMut for TextLayer {
    fn as_mut_ptr(&self) -> *mut types::Layer {
        interface::text_layer_get_layer(self.internal)
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        interface::text_layer_destroy(self.internal);
    }
}
