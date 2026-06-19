/*
 * This file is part of pebble-rs.
 * Copyright (c) 2026 Peatral
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
use crate::graphics::bitmap::{BitmapRef, IBitmap};
use crate::graphics::types::Color;
use crate::layer::{ILayer, ILayerMut};
use crate::pebble::clicks::{ClickDelegate, trampoline_click_config_provider};
use crate::pebble::window::WindowRef;
use alloc::boxed::Box;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};
use pebble_sys::Layer;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct ActionBarLayerRef {
    internal: *mut pebble_sys::ActionBarLayer,
}

impl ActionBarLayerRef {
    /// Sets an action bar icon onto one of the 3 slots as identified by button_id.
    pub fn set_icon(&self, button_id: pebble_sys::ButtonId, icon: BitmapRef) {
        unsafe {
            pebble_sys::action_bar_layer_set_icon(self.internal, button_id, icon.as_ptr());
        }
    }

    /// Convenience function to clear out an existing icon.
    pub fn clear_icon(&self, button_id: pebble_sys::ButtonId) {
        unsafe {
            pebble_sys::action_bar_layer_clear_icon(self.internal, button_id);
        }
    }

    /// Adds the action bar's layer on top of the window's root layer and adjusts the layout.
    pub fn add_to_window(&self, window: &WindowRef) {
        unsafe {
            pebble_sys::action_bar_layer_add_to_window(self.internal, window.as_ptr());
        }
    }

    /// Removes the action bar from the window and unconfigures the window's click configuration provider.
    pub fn remove_from_window(&self) {
        unsafe {
            pebble_sys::action_bar_layer_remove_from_window(self.internal);
        }
    }

    /// Sets the background color of the action bar. Defaults to GColorBlack.
    pub fn set_background_color(&self, background_color: Color) {
        unsafe {
            pebble_sys::action_bar_layer_set_background_color(self.internal, background_color.0);
        }
    }

    /// Sets an action bar icon onto one of the 3 slots with an optional animation.
    pub fn set_icon_animated(
        &self,
        button_id: pebble_sys::ButtonId,
        icon: BitmapRef,
        animated: bool,
    ) {
        unsafe {
            pebble_sys::action_bar_layer_set_icon_animated(
                self.internal,
                button_id,
                icon.as_ptr(),
                animated,
            );
        }
    }

    /// Sets the animation to use while a button is pressed on an ActionBarLayer.
    pub fn set_icon_press_animation(
        &self,
        button_id: pebble_sys::ButtonId,
        animation: pebble_sys::ActionBarLayerIconPressAnimation,
    ) {
        unsafe {
            pebble_sys::action_bar_layer_set_icon_press_animation(
                self.internal,
                button_id,
                animation,
            );
        }
    }
}

impl ILayer for ActionBarLayerRef {
    fn as_ptr(&self) -> *const Layer {
        unsafe { pebble_sys::action_bar_layer_get_layer(self.internal) }
    }
}

impl ILayerMut for ActionBarLayerRef {
    fn as_mut_ptr(&self) -> *mut Layer {
        unsafe { pebble_sys::action_bar_layer_get_layer(self.internal) }
    }
}

impl From<*mut pebble_sys::ActionBarLayer> for ActionBarLayerRef {
    fn from(internal: *mut pebble_sys::ActionBarLayer) -> Self {
        Self { internal }
    }
}

/// A vertical, bar-shaped control widget on the right edge of the window.
pub struct ActionBarLayer<T: ClickDelegate> {
    layer_ref: ActionBarLayerRef,
    delegate: Box<T>,
}

impl<T: ClickDelegate> ActionBarLayer<T> {
    /// Creates a new ActionBarLayer on the heap and initializes it with the given delegate.
    pub fn new(delegate: T) -> Self {
        unsafe {
            let internal = pebble_sys::action_bar_layer_create();

            let layer = Self {
                layer_ref: internal.into(),
                delegate: Box::new(delegate),
            };

            let context_ptr = &*layer.delegate as *const T as *mut c_void;

            pebble_sys::action_bar_layer_set_context(internal, context_ptr);
            pebble_sys::action_bar_layer_set_click_config_provider(
                internal,
                Some(trampoline_click_config_provider::<T>),
            );

            layer
        }
    }
}

impl<T: ClickDelegate> ILayer for ActionBarLayer<T> {
    fn as_ptr(&self) -> *const Layer {
        self.layer_ref.as_ptr()
    }
}

impl<T: ClickDelegate> ILayerMut for ActionBarLayer<T> {
    fn as_mut_ptr(&self) -> *mut Layer {
        self.layer_ref.as_mut_ptr()
    }
}

impl<T: ClickDelegate> Deref for ActionBarLayer<T> {
    type Target = ActionBarLayerRef;

    fn deref(&self) -> &Self::Target {
        &self.layer_ref
    }
}

impl<T: ClickDelegate> DerefMut for ActionBarLayer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.layer_ref
    }
}

impl<T: ClickDelegate> Drop for ActionBarLayer<T> {
    fn drop(&mut self) {
        unsafe {
            pebble_sys::action_bar_layer_destroy(self.internal);
        }
    }
}
