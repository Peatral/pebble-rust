use alloc::boxed::Box;
use core::ffi::c_void;

use crate::layer::ILayer;
use crate::pebble::internal::functions::interface;
use crate::pebble::internal::types;
use crate::pebble::types::{Bitmap, GColor, GRect};
use crate::pebble::window::WindowRef;

// Import the ClickDelegate and the master trampoline we defined earlier
use crate::pebble::clicks::{trampoline_click_config_provider, ClickDelegate};

/// A vertical, bar-shaped control widget on the right edge of the window.
pub struct ActionBarLayer<T: ClickDelegate> {
    internal: *mut types::ActionBarLayer,
    inner: *mut types::Layer,
    // The delegate is boxed so it lives exactly as long as the layer
    delegate: Box<T>,
}

impl<T: ClickDelegate> ActionBarLayer<T> {
    /// Creates a new ActionBarLayer on the heap and initializes it with the given delegate.
    pub fn new(delegate: T) -> Self {
        let internal = interface::action_bar_layer_create();
        let inner = interface::action_bar_layer_get_layer(internal);

        let layer = Self {
            internal,
            inner,
            delegate: Box::new(delegate),
        };

        // Extract the stable pointer from the Box to use as our C context
        let context_ptr = &*layer.delegate as *const T as *mut c_void;

        // Automatically configure the context and route the click provider to our master trampoline
        interface::action_bar_layer_set_context(layer.internal, context_ptr);
        interface::action_bar_layer_set_click_config_provider(
            layer.internal,
            trampoline_click_config_provider::<T>,
        );

        layer
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

impl<T: ClickDelegate> Drop for ActionBarLayer<T> {
    fn drop(&mut self) {
        interface::action_bar_layer_destroy(self.internal);
    }
}

impl<T: ClickDelegate> ILayer for ActionBarLayer<T> {
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
