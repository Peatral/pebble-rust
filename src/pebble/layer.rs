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
pub use menu_layer::{MenuIndexRef, MenuLayer, MenuLayerDelegate, MenuLayerRef};
pub use status_bar_layer::StatusBarLayer;

/// A safe, immutable reference to a standard UI Layer.
/// Used primarily for reading properties of layers you don't own (like the menu background).
pub struct LayerRef {
    internal: *const types::Layer,
}

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
    fn get_internal(&self) -> *mut types::Layer;

    fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.get_internal())
    }
    fn set_bounds(&self, bounds: GRect) {
        interface::layer_set_bounds(self.get_internal(), bounds);
    }
    fn get_unobstructed_bounds(&self) -> GRect {
        interface::layer_get_unobstructed_bounds(self.get_internal())
    }

    fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.get_internal())
    }
    fn set_frame(&self, frame: GRect) {
        interface::layer_set_frame(self.get_internal(), frame);
    }

    fn add_child(&self, child: &dyn ILayer) {
        interface::layer_add_child(self.get_internal(), child.get_internal());
    }
    fn remove_from_parent(&self) {
        interface::layer_remove_from_parent(self.get_internal());
    }
    fn remove_child_layers(&self) {
        interface::layer_remove_child_layers(self.get_internal());
    }
    fn insert_below_sibling(&self, sibling: &dyn ILayer) {
        interface::layer_insert_below_sibling(self.get_internal(), sibling.get_internal());
    }
    fn insert_above_sibling(&self, sibling: &dyn ILayer) {
        interface::layer_insert_above_sibling(self.get_internal(), sibling.get_internal());
    }

    fn mark_dirty(&self) {
        interface::layer_mark_dirty(self.get_internal());
    }

    fn set_hidden(&self, hidden: bool) {
        interface::layer_set_hidden(self.get_internal(), hidden);
    }
    fn get_hidden(&self) -> bool {
        interface::layer_get_hidden(self.get_internal())
    }

    fn set_clips(&self, clips: bool) {
        interface::layer_set_clips(self.get_internal(), clips);
    }
    fn get_clips(&self) -> bool {
        interface::layer_get_clips(self.get_internal())
    }
}

impl LayerRef {
    pub(crate) fn from_ptr(ptr: *const types::Layer) -> Self {
        Self { internal: ptr }
    }

    pub fn get_bounds(&self) -> GRect {
        interface::layer_get_bounds(self.internal)
    }

    pub fn get_unobstructed_bounds(&self) -> GRect {
        interface::layer_get_unobstructed_bounds(self.internal)
    }

    pub fn get_frame(&self) -> GRect {
        interface::layer_get_frame(self.internal)
    }

    pub fn get_hidden(&self) -> bool {
        interface::layer_get_hidden(self.internal)
    }
    pub fn get_clips(&self) -> bool {
        interface::layer_get_clips(self.internal)
    }
}

impl ILayer for Layer {
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
