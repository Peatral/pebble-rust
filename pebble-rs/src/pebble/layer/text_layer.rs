use crate::graphics::types::{Color, Rect, Size};
use crate::layer::{ILayer, ILayerMut};
use crate::system::fonts::Font;
use alloc::ffi::CString;
use core::ffi::CStr;
use pebble_sys::{GTextAlignment, GTextOverflowMode};

pub struct TextLayer {
    internal: *mut pebble_sys::TextLayer,
    _text_buffer: Option<CString>,
}

impl TextLayer {
    /// Creates a new TextLayer on the heap and initializes it.
    pub fn new(bounds: Rect) -> TextLayer {
        unsafe {
            let internal = pebble_sys::text_layer_create(bounds.0);

            TextLayer {
                internal,
                _text_buffer: None,
            }
        }
    }

    /// Sets the text using a static C-String (e.g., c"Hello World").
    /// This is the most memory-efficient way to set text.
    pub fn set_text_static(&self, text: &'static CStr) {
        unsafe {
            pebble_sys::text_layer_set_text(self.internal, text.as_ptr());
        }
    }

    /// Sets the text using a dynamically allocated CString.
    /// The Layer takes ownership of the string to guarantee its lifetime.
    pub fn set_text(&mut self, text: CString) {
        unsafe {
            pebble_sys::text_layer_set_text(self.internal, text.as_ptr());
        }
        self._text_buffer = Some(text);
    }

    /// Safely retrieves the string currently displayed by the TextLayer.
    pub fn get_text(&self) -> Option<&str> {
        unsafe {
            let ptr = pebble_sys::text_layer_get_text(self.internal);
            if ptr.is_null() {
                None
            } else {
                // Safely cast the C string back to a Rust string slice
                Some(CStr::from_ptr(ptr).to_str().unwrap_or(""))
            }
        }
    }

    pub fn set_background_color(&self, color: Color) {
        unsafe {
            pebble_sys::text_layer_set_background_color(self.internal, color.0);
        }
    }

    pub fn set_text_color(&self, color: Color) {
        unsafe {
            pebble_sys::text_layer_set_text_color(self.internal, color.0);
        }
    }

    pub fn set_font(&self, font: Font) {
        unsafe {
            pebble_sys::text_layer_set_font(self.internal, font.internal);
        }
    }

    pub fn set_text_alignment(&self, alignment: GTextAlignment) {
        unsafe {
            pebble_sys::text_layer_set_text_alignment(self.internal, alignment);
        }
    }

    pub fn set_overflow_mode(&self, line_mode: GTextOverflowMode) {
        unsafe {
            pebble_sys::text_layer_set_overflow_mode(self.internal, line_mode);
        }
    }

    /// Enables text flow following the boundaries of the screen.
    /// Crucial for Pebble Time Round (circular displays).
    /// Note: The TextLayer must be added to the view hierarchy BEFORE calling this!
    pub fn enable_screen_text_flow_and_paging(&self, inset: u8) {
        unsafe {
            pebble_sys::text_layer_enable_screen_text_flow_and_paging(self.internal, inset);
        }
    }

    /// Restores text flow and paging to the standard rectangular defaults.
    pub fn restore_default_text_flow_and_paging(&self) {
        unsafe {
            pebble_sys::text_layer_restore_default_text_flow_and_paging(self.internal);
        }
    }

    /// Calculates the minimum bounding box needed to fit the current text.
    pub fn get_content_size(&self) -> Size {
        unsafe { Size(pebble_sys::text_layer_get_content_size(self.internal)) }
    }

    /// Convenience function to quickly update the frame size of the TextLayer.
    pub fn set_size(&self, max_size: Size) {
        unsafe {
            pebble_sys::text_layer_set_size(self.internal, max_size.0);
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
