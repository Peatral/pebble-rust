/*
 * This file is part of pebble-rust.
 * Copyright (c) 2019 RoccoDev
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

use crate::pebble::internal::functions::declarations::text_layer_set_text;
use crate::pebble::internal::types::GTextAlignment;
use crate::pebble::internal::{functions::interface, types};
use crate::pebble::types::{Bitmap, GCompOp, GRect};
use crate::system::fonts::Font;
use core::ffi::{CStr, c_char};

pub mod action_bar_layer;
pub mod menu_layer;
pub mod status_bar_layer;

pub use action_bar_layer::ActionBarLayer;
pub use menu_layer::{MenuLayer, MenuLayerDelegate, MenuLayerRef};
pub use status_bar_layer::StatusBarLayer;

pub struct Layer {
    internal: *mut types::Layer,
    is_owned_by_rust: bool,
}

pub struct TextLayer {
    internal: *mut types::TextLayer,
    inner: *mut types::Layer,
}

pub struct BitmapLayer {
    internal: *mut types::BitmapLayer,
    inner: *mut types::Layer,
}

pub trait ILayer {
    fn get_bounds(&self) -> GRect;
    fn get_frame(&self) -> GRect;
    fn add_child(&self, layer: &dyn ILayer);
    fn mark_dirty(&self);
    fn get_internal(&self) -> *mut types::Layer;
}

impl ILayer for Layer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.internal)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.internal)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.internal, layer.get_internal())
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.internal);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.internal
    }
}

impl Layer {
    pub fn new(bounds: GRect) -> Layer {
        Layer {
            internal: interface::layer_create(bounds),
            is_owned_by_rust: false,
        }
    }

    pub(crate) fn from_ptr(ptr: *mut types::Layer, is_owned_by_rust: bool) -> Layer {
        Layer {
            internal: ptr,
            is_owned_by_rust,
        }
    }
}

impl ILayer for TextLayer {
    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.inner)
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.inner)
    }

    fn add_child(&self, layer: &dyn ILayer) {
        interface::layer_add_child(self.inner, layer.get_internal());
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.inner);
    }

    fn get_internal(&self) -> *mut types::Layer {
        self.inner
    }
}

impl Drop for Layer {
    fn drop(&mut self) {
        if self.is_owned_by_rust {
            interface::layer_destroy(self.internal);
        }
    }
}

impl TextLayer {
    pub fn new(bounds: GRect) -> TextLayer {
        let internal = interface::text_layer_create(bounds);
        let inner = interface::text_layer_get_layer(internal);

        TextLayer { internal, inner }
    }

    pub fn set_text(&self, text: &CStr) {
        interface::text_layer_set_text(self.internal, text);
    }
    pub unsafe fn set_text_raw(&self, text: *const c_char) {
        unsafe {
            text_layer_set_text(self.internal, text);
        }
    }

    pub fn set_font(&self, font: Font) {
        interface::text_layer_set_font(self.internal, font.internal)
    }
    pub fn set_text_alignment(&self, alignment: GTextAlignment) {
        interface::text_layer_set_text_alignment(self.internal, alignment);
    }
}

impl Drop for TextLayer {
    fn drop(&mut self) {
        interface::text_layer_destroy(self.internal);
    }
}

impl ILayer for BitmapLayer {
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

impl BitmapLayer {
    pub fn new(bounds: GRect) -> BitmapLayer {
        let internal = interface::bitmap_layer_create(bounds);
        let inner = interface::bitmap_layer_get_layer(internal);

        BitmapLayer { internal, inner }
    }

    pub fn set_bitmap(&self, bitmap: &Bitmap) {
        interface::bitmap_layer_set_bitmap(self.internal, bitmap.internal);
    }

    pub fn set_compositing_mode(&self, mode: GCompOp) {
        interface::bitmap_layer_set_compositing_mode(self.internal, mode);
    }
}

impl Drop for BitmapLayer {
    fn drop(&mut self) {
        interface::bitmap_layer_destroy(self.internal);
    }
}
