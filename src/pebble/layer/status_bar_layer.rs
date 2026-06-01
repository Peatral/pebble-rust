use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::internal::types::StatusBarLayerSeparatorMode;
use crate::pebble::types::{GColor, GRect};

/// A layer that serves as a configurable status bar.
pub struct StatusBarLayer {
    internal: *mut types::StatusBarLayer,
    inner: *mut types::Layer,
}

impl StatusBarLayer {
    /// Creates a new StatusBarLayer on the heap and initializes it with the default values.
    /// (Text color: GColorBlack, Background color: GColorWhite)
    pub fn new() -> Self {
        let internal = interface::status_bar_layer_create();
        let inner = interface::status_bar_layer_get_layer(internal);

        Self { internal, inner }
    }

    /// Gets background color of StatusBarLayer.
    pub fn get_background_color(&self) -> GColor {
        interface::status_bar_layer_get_background_color(self.internal)
    }

    /// Gets foreground color of StatusBarLayer.
    pub fn get_foreground_color(&self) -> GColor {
        interface::status_bar_layer_get_foreground_color(self.internal)
    }

    /// Sets the background and foreground colors of StatusBarLayer.
    pub fn set_colors(&self, background: GColor, foreground: GColor) {
        interface::status_bar_layer_set_colors(self.internal, background, foreground);
    }

    /// Sets the mode of the StatusBarLayer separator, to help divide it from content.
    pub fn set_separator_mode(&self, mode: StatusBarLayerSeparatorMode) {
        interface::status_bar_layer_set_separator_mode(self.internal, mode);
    }
}

impl Default for StatusBarLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for StatusBarLayer {
    fn drop(&mut self) {
        interface::status_bar_layer_destroy(self.internal);
    }
}

impl ILayer for StatusBarLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner)
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}
