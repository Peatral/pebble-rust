use crate::layer::{ILayer, ILayerMut};
use pebble_sys::{GColor, Layer, StatusBarLayerSeparatorMode};

/// A layer that serves as a configurable status bar.
#[repr(transparent)]
pub struct StatusBarLayer {
    internal: *mut pebble_sys::StatusBarLayer,
}

impl StatusBarLayer {
    /// Creates a new StatusBarLayer on the heap and initializes it with the default values.
    /// (Text color: GColorBlack, Background color: GColorWhite)
    pub fn new() -> Self {
        unsafe {
            let internal = pebble_sys::status_bar_layer_create();

            Self { internal }
        }
    }

    /// Gets background color of StatusBarLayer.
    pub fn get_background_color(&self) -> GColor {
        unsafe { pebble_sys::status_bar_layer_get_background_color(self.internal) }
    }

    /// Gets foreground color of StatusBarLayer.
    pub fn get_foreground_color(&self) -> GColor {
        unsafe { pebble_sys::status_bar_layer_get_foreground_color(self.internal) }
    }

    /// Sets the background and foreground colors of StatusBarLayer.
    pub fn set_colors(&self, background: GColor, foreground: GColor) {
        unsafe {
            pebble_sys::status_bar_layer_set_colors(self.internal, background, foreground);
        }
    }

    /// Sets the mode of the StatusBarLayer separator, to help divide it from content.
    pub fn set_separator_mode(&self, mode: StatusBarLayerSeparatorMode) {
        unsafe {
            pebble_sys::status_bar_layer_set_separator_mode(self.internal, mode);
        }
    }
}

impl Default for StatusBarLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ILayer for StatusBarLayer {
    fn as_ptr(&self) -> *const Layer {
        unsafe { pebble_sys::status_bar_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for StatusBarLayer {
    fn as_mut_ptr(&self) -> *mut Layer {
        unsafe { pebble_sys::status_bar_layer_get_layer(self.internal) }
    }
}

impl Drop for StatusBarLayer {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::status_bar_layer_destroy(self.internal);
        }
    }
}
