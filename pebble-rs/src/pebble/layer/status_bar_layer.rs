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
use crate::graphics::types::Color;
use crate::layer::{ILayer, ILayerMut};
use pebble_sys::{Layer, StatusBarLayerSeparatorMode};

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
    pub fn get_background_color(&self) -> Color {
        Color(unsafe { pebble_sys::status_bar_layer_get_background_color(self.internal) })
    }

    /// Gets foreground color of StatusBarLayer.
    pub fn get_foreground_color(&self) -> Color {
        Color(unsafe { pebble_sys::status_bar_layer_get_foreground_color(self.internal) })
    }

    /// Sets the background and foreground colors of StatusBarLayer.
    pub fn set_colors(&self, background: Color, foreground: Color) {
        unsafe {
            pebble_sys::status_bar_layer_set_colors(self.internal, background.0, foreground.0);
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
