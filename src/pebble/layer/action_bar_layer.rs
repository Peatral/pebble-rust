use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::types::{Bitmap, GColor, GRect};
use crate::pebble::window::WindowRef;
use core::ffi::c_void;

/// A vertical, bar-shaped control widget on the right edge of the window.
pub struct ActionBarLayer {
    internal: *mut types::ActionBarLayer,
    inner: *mut types::Layer,
}

impl ActionBarLayer {
    /// Creates a new ActionBarLayer on the heap and initializes it with the default values.
    pub fn new() -> Self {
        let internal = interface::action_bar_layer_create();
        let inner = interface::action_bar_layer_get_layer(internal);

        Self { internal, inner }
    }

    /// Sets the context parameter, which will be passed in to ClickHandler callbacks
    /// and the ClickConfigProvider callback of the action bar.
    pub fn set_context(&self, context: *mut c_void) {
        interface::action_bar_layer_set_context(self.internal, context);
    }

    /// Sets the click configuration provider callback of the action bar.
    pub fn set_click_config_provider(&self, click_config_provider: types::ClickConfigProvider) {
        interface::action_bar_layer_set_click_config_provider(self.internal, click_config_provider);
    }

    /// Sets an action bar icon onto one of the 3 slots as identified by button_id.
    pub fn set_icon(&self, button_id: types::ButtonId, icon: &Bitmap) {
        interface::action_bar_layer_set_icon(self.internal, button_id, icon.internal);
    }

    /// Convenience function to clear out an existing icon.
    pub fn clear_icon(&self, button_id: types::ButtonId) {
        interface::action_bar_layer_clear_icon(self.internal, button_id);
    }

    /// Adds the action bar's layer on top of the window's root layer and adjusts the layout.
    pub fn add_to_window(&self, window: &WindowRef) {
        interface::action_bar_layer_add_to_window(self.internal, window.as_ptr());
    }

    /// Removes the action bar from the window and unconfigures the window's click configuration provider.
    pub fn remove_from_window(&self) {
        interface::action_bar_layer_remove_from_window(self.internal);
    }

    /// Sets the background color of the action bar. Defaults to GColorBlack.
    pub fn set_background_color(&self, background_color: GColor) {
        interface::action_bar_layer_set_background_color(self.internal, background_color);
    }

    /// Sets an action bar icon onto one of the 3 slots with an optional animation.
    pub fn set_icon_animated(&self, button_id: types::ButtonId, icon: &Bitmap, animated: bool) {
        interface::action_bar_layer_set_icon_animated(
            self.internal,
            button_id,
            icon.internal,
            animated,
        );
    }

    /// Sets the animation to use while a button is pressed on an ActionBarLayer.
    pub fn set_icon_press_animation(
        &self,
        button_id: types::ButtonId,
        animation: types::ActionBarLayerIconPressAnimation,
    ) {
        interface::action_bar_layer_set_icon_press_animation(self.internal, button_id, animation);
    }
}

impl Drop for ActionBarLayer {
    fn drop(&mut self) {
        interface::action_bar_layer_destroy(self.internal);
    }
}

impl ILayer for ActionBarLayer {
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
